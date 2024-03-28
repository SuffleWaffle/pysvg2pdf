// >>>> </> STANDARD IMPORTS </>
// >>>> ********************************************************************************
// use std::error::Error;
// use std::time::{Duration, Instant};
// >>>> ********************************************************************************

// >>>> </> EXTERNAL IMPORTS </>
// >>>> ********************************************************************************
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use svg2pdf;
use svg2pdf::usvg::{fontdb, PostProcessingSteps, TreeParsing, TreePostProc};
// use usvg;
// >>>> ********************************************************************************

// ________________________________________________________________________________
// --- Converts the SVG file to PDF file ---w
#[pyfunction]
fn get_pdf_path_from_svg_path(
    input_filepath: String,
    output_filepath: String) -> PyResult<()> {
    // ________________________________________________________________________________
    // Read the SVG file into a string
    let svg_string: String = std::fs::read_to_string(&input_filepath)
        .map_err(PyErr::new::<pyo3::exceptions::PyIOError, _>)?;

    // ________________________________________________________________________________
    // - Create usvg::Options with default settings for parsing the SVG string to a usvg::Tree
    let options = svg2pdf::usvg::Options::default();

    // - Create usvg::Tree from the SVG string
    let mut svg_tree = svg2pdf::usvg::Tree::from_str(&svg_string, &options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // - Create a font database and load system fonts
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    svg_tree.postprocess(PostProcessingSteps::default(), &db);

    // ________________________________________________________________________________
    // - Convert the usvg:Tree to a PDF Vec<u8>
    let pdf_bytes = svg2pdf::convert_tree(&svg_tree, svg2pdf::Options::default());

    // ________________________________________________________________________________
    // Write to a file
    std::fs::write(&output_filepath, pdf_bytes)
        .map_err(PyErr::new::<pyo3::exceptions::PyIOError, _>)?;

    // Stop the timer
    // let duration: Duration = start.elapsed();
    // println!("- FILEPATH: {:} \n- TIME: {:?}", &input_filepath, duration);

    Ok(())
}

// ________________________________________________________________________________
// --- Converts the SVG string to a PDF Vec<u8>
#[pyfunction]
fn get_pdf_bytes_from_svg_string(
    svg_string: String) -> PyResult<Vec<u8>> {
    // - Start the timer
    // let start: Instant = Instant::now();

    // ________________________________________________________________________________
    // - Create usvg::Options with default settings
    let options = svg2pdf::usvg::Options::default();
    
    // - Create usvg::Tree from the SVG string
    let mut svg_tree = svg2pdf::usvg::Tree::from_str(&svg_string, &options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // - Create a font database and load system fonts
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    svg_tree.postprocess(PostProcessingSteps::default(), &db);

    // ________________________________________________________________________________
    // - Convert the usvg:Tree to a PDF Vec<u8>
    let pdf_bytes = svg2pdf::convert_tree(&svg_tree, svg2pdf::Options::default());

    // - Stop the timer
    // let duration: Duration = start.elapsed();
    // println!("- SVG-to-PDF TIME: {:?}", duration);

    Ok(pdf_bytes)
}

// ________________________________________________________________________________
// --- Python module setup ---
#[pymodule]
fn pysvg2pdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_pdf_path_from_svg_path, m)?)?;
    m.add_function(wrap_pyfunction!(get_pdf_bytes_from_svg_string, m)?)?;
    Ok(())
}
