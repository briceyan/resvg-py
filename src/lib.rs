use pyo3::{exceptions::*, prelude::*, types::*};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tiny_skia::{Color, Pixmap, Transform};
use usvg;

#[pyclass(unsendable)]
struct Options {
    inner: usvg::Options,
}

#[pymethods]
impl Options {
    #[staticmethod]
    fn default() -> PyResult<Self> {
        let options = usvg::Options::default();
        Ok(Options { inner: options })
    }

    #[getter]
    fn get_resources_dir(&self) -> PyResult<Option<&str>> {
        if let Some(rd) = &self.inner.resources_dir {
            Ok(rd.to_str())
        } else {
            Ok(None)
        }
    }

    #[setter]
    fn set_resources_dir(&mut self, value: &str) -> PyResult<()> {
        let path = PathBuf::from_str(value).map_err(|e| PyIOError::new_err(e.to_string()))?;
        self.inner.resources_dir = Some(path);
        Ok(())
    }

    #[getter]
    fn get_dpi(&self) -> PyResult<f32> {
        Ok(self.inner.dpi)
    }

    #[setter]
    fn set_dpi(&mut self, value: f32) -> PyResult<()> {
        self.inner.dpi = value;
        Ok(())
    }

    #[getter]
    fn get_font_family(&self) -> PyResult<&str> {
        Ok(&self.inner.font_family)
    }

    #[setter]
    fn set_font_family(&mut self, value: &str) -> PyResult<()> {
        self.inner.font_family = value.to_string();
        Ok(())
    }

    #[getter]
    fn get_font_size(&self) -> PyResult<f32> {
        Ok(self.inner.font_size)
    }

    #[setter]
    fn set_font_size(&mut self, value: f32) -> PyResult<()> {
        self.inner.font_size = value;
        Ok(())
    }

    // pub resources_dir: Option<PathBuf>,
    // pub dpi: f32,
    // pub font_family: String,
    // pub font_size: f32,
    // pub languages: Vec<String>,
    // pub shape_rendering: ShapeRendering,
    // pub text_rendering: TextRendering,
    // pub image_rendering: ImageRendering,
    // pub default_size: Size,
    // pub image_href_resolver: ImageHrefResolver,
}

#[pyclass(unsendable)]
struct FontDatabase {
    inner: usvg::fontdb::Database,
}

#[pymethods]
impl FontDatabase {
    #[staticmethod]
    fn default() -> PyResult<Self> {
        let db = usvg::fontdb::Database::default();
        Ok(FontDatabase { inner: db })
    }

    fn load_font_file(&mut self, file: &str) -> PyResult<()> {
        self.inner
            .load_font_file(Path::new(file))
            .map_err(|e| PyIOError::new_err(e.to_string()))
    }

    fn load_fonts_dir(&mut self, dir: &str) -> PyResult<()> {
        self.inner.load_fonts_dir(Path::new(dir));
        Ok(())
    }

    fn load_font_data(&mut self, data: Vec<u8>) -> PyResult<()> {
        self.inner.load_font_data(data);
        Ok(())
    }

    fn load_system_fonts(&mut self) -> PyResult<()> {
        self.inner.load_system_fonts();
        Ok(())
    }
}

#[pyclass(unsendable)]
struct Tree {
    inner: usvg::Tree,
}

#[pymethods]
impl Tree {
    #[staticmethod]
    fn from_str(svg: &str, opts: &Options, fontdb: &FontDatabase) -> PyResult<Self> {
        let tree = usvg::Tree::from_str(svg, &opts.inner, &fontdb.inner).unwrap();
        Ok(Tree { inner: tree })
    }

    fn int_size(&self) -> PyResult<(u32, u32)> {
        let sz = self.inner.size().to_int_size();
        Ok((sz.width(), sz.height()))
    }
}

#[pyfunction]
#[pyo3(signature = (tree, transform, bg_file=None, bg_data=None, bg_size=None, bg_color=None))]
fn render(
    tree: &Tree,
    transform: &PyTuple,
    bg_file: Option<String>,
    bg_data: Option<Vec<u8>>,
    bg_size: Option<&PyTuple>,
    bg_color: Option<&PyTuple>,
) -> PyResult<Vec<u8>> {
    let mut pixmap: Pixmap;
    if let Some(bg_file) = bg_file {
        if bg_data.is_some() || bg_size.is_some() || bg_color.is_some() {
            return Err(PyValueError::new_err(
                "bg_data, bg_size, bg_color are invalid when bg_file is set",
            ));
        }
        pixmap = Pixmap::load_png(bg_file).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    } else if let Some(bg_data) = bg_data {
        if bg_size.is_some() || bg_color.is_some() {
            return Err(PyValueError::new_err(
                "bg_size, bg_color are invalid when bg_data is set",
            ));
        }
        pixmap =
            Pixmap::decode_png(&bg_data).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    } else {
        let (w, h);
        if let Some(bg_size) = bg_size {
            let sz: (u32, u32) = bg_size.extract()?;
            w = sz.0;
            h = sz.1;
        } else {
            let sz = tree.inner.size().to_int_size();
            w = sz.width();
            h = sz.height();
        }
        pixmap =
            Pixmap::new(w, h).ok_or_else(|| PyRuntimeError::new_err("failed to create pixmap"))?;
        if let Some(bg_color) = bg_color {
            let (r, g, b, a): (u8, u8, u8, u8) = bg_color.extract()?;
            pixmap.fill(Color::from_rgba8(r, g, b, a));
        }
    }

    let (a, b, c, d, e, f) = transform.extract()?;
    let tr = Transform::from_row(a, d, b, e, c, f);
    resvg::render(&tree.inner, tr, &mut pixmap.as_mut());
    pixmap
        .encode_png()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

#[pymodule]
#[pyo3(name = "resvg")]
fn resvg_module(py: Python, m: &PyModule) -> PyResult<()> {
    // usvg submodule
    let usvg_module = PyModule::new(py, "usvg")?;
    usvg_module.add_class::<Tree>()?;
    usvg_module.add_class::<Options>()?;
    usvg_module.add_class::<FontDatabase>()?;
    // resvg module
    m.add_submodule(usvg_module)?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}
