use std::{path::PathBuf, str::FromStr};

#[allow(unused_imports)]
use pyo3::pymodule;
use pyo3::{
    exceptions::{PyIOError, PyRuntimeError, PyValueError},
    prelude::{Bound, PyResult, pyclass, pyfunction, pymethods},
    types::{PyAnyMethods, PyTuple},
};
use tiny_skia::{Color, Pixmap, Transform};
use usvg::{Options as UsvgOptions, Tree as UsvgTree};

mod vendored;

#[pyclass(unsendable)]
struct Options {
    inner: UsvgOptions<'static>,
}

#[pymethods]
impl Options {
    #[staticmethod]
    fn default() -> Self {
        let options = UsvgOptions::default();
        Options { inner: options }
    }

    #[getter]
    fn get_resources_dir(&self) -> Option<&str> {
        self.inner.resources_dir.as_ref().and_then(|rd| rd.to_str())
    }

    #[setter]
    fn set_resources_dir(&mut self, value: &str) -> PyResult<()> {
        let path = PathBuf::from_str(value).map_err(|e| PyIOError::new_err(e.to_string()))?;
        self.inner.resources_dir = Some(path);
        Ok(())
    }

    #[getter]
    fn get_dpi(&self) -> f32 {
        self.inner.dpi
    }

    #[setter]
    fn set_dpi(&mut self, value: f32) -> PyResult<()> {
        if !(10.0..=4000.0).contains(&value) {
            return Err(PyValueError::new_err("DPI must be between 10 and 4000"));
        }
        self.inner.dpi = value;
        Ok(())
    }

    #[getter]
    fn get_font_family(&self) -> &str {
        self.inner.font_family.as_str()
    }

    #[setter]
    fn set_font_family(&mut self, value: &str) {
        self.inner.font_family = value.to_string();
    }

    #[getter]
    fn get_font_size(&self) -> f32 {
        self.inner.font_size
    }

    #[setter]
    fn set_font_size(&mut self, value: f32) -> PyResult<()> {
        if !(1.0..=192.0).contains(&value) {
            return Err(PyValueError::new_err("Font size must be between 1 and 192"));
        }
        self.inner.font_size = value;
        Ok(())
    }

    fn load_system_fonts(&mut self) {
        self.inner.fontdb_mut().load_system_fonts();
    }

    fn load_font_file(&mut self, path: &str) -> PyResult<()> {
        self.inner
            .fontdb_mut()
            .load_font_file(path)
            .map_err(|e| PyIOError::new_err(e.to_string()))
    }

    fn load_fonts_dir(&mut self, path: &str) {
        self.inner.fontdb_mut().load_fonts_dir(path);
    }

    fn set_serif_family(&mut self, family: &str) {
        self.inner.fontdb_mut().set_serif_family(family);
    }

    fn set_sans_serif_family(&mut self, family: &str) {
        self.inner.fontdb_mut().set_sans_serif_family(family);
    }

    fn set_cursive_family(&mut self, family: &str) {
        self.inner.fontdb_mut().set_cursive_family(family);
    }

    fn set_fantasy_family(&mut self, family: &str) {
        self.inner.fontdb_mut().set_fantasy_family(family);
    }

    fn set_monospace_family(&mut self, family: &str) {
        self.inner.fontdb_mut().set_monospace_family(family);
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
struct Tree {
    inner: UsvgTree,
}

#[pymethods]
impl Tree {
    #[staticmethod]
    fn from_str(svg: &str, opts: &Options) -> PyResult<Self> {
        let tree = UsvgTree::from_str(svg, &opts.inner)
            .map_err(|e| PyValueError::new_err(format!("Invalid SVG: {e}")))?;
        Ok(Tree { inner: tree })
    }

    fn int_size(&self) -> (u32, u32) {
        let sz = self.inner.size().to_int_size();
        (sz.width(), sz.height())
    }
}

#[pyfunction]
#[pyo3(signature = (tree, transform, bg_file=None, bg_data=None, bg_size=None, bg_color=None))]
fn render<'py>(
    tree: &Tree,
    transform: &Bound<'py, PyTuple>,
    bg_file: Option<String>,
    bg_data: Option<Vec<u8>>,
    bg_size: Option<&Bound<'py, PyTuple>>,
    bg_color: Option<&Bound<'py, PyTuple>>,
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
        let (width, height);
        if let Some(bg_size) = bg_size {
            let sz: (u32, u32) = bg_size.extract()?;
            width = sz.0;
            height = sz.1;
        } else {
            let sz = tree.inner.size().to_int_size();
            width = sz.width();
            height = sz.height();
        }
        pixmap = Pixmap::new(width, height)
            .ok_or_else(|| PyRuntimeError::new_err("failed to create pixmap"))?;
        if let Some(bg_color) = bg_color {
            let (r, g, b, a): (u8, u8, u8, u8) = bg_color.extract()?;
            pixmap.fill(Color::from_rgba8(r, g, b, a));
        }
    }

    let (scale_x, skew_x, translate_x, skew_y, scale_y, translate_y) = transform.extract()?;
    let tr = Transform::from_row(scale_x, skew_y, skew_x, scale_y, translate_x, translate_y);
    resvg::render(&tree.inner, tr, &mut pixmap.as_mut());
    pixmap
        .encode_png()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

#[pymodule(name = "_resvg")]
mod resvg_module {
    use std::ffi::OsString;

    use pyo3::pyfunction;

    #[pymodule_export]
    use super::render;
    #[pymodule_export]
    use super::usvg_module;
    use crate::vendored::resvg_main::process;

    #[pyfunction]
    fn _script_entrypoint(env_args: Vec<OsString>) -> u8 {
        match process(env_args) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("Error: {e}.");
                1
            }
        }
    }
}

#[pymodule(name = "usvg")]
mod usvg_module {
    // usvg submodule
    #[pymodule_export]
    use super::Options;
    #[pymodule_export]
    use super::Tree;
}
