# pysvg2pdf

_**Blazingly Fast**_ :tm: SVG to PDF file conversion for Python.

This project is based on Rust's [svg2pdf](https://github.com/typst/svg2pdf) and [resvg](https://github.com/RazrFalcon/resvg) projects.

The project uses pyo3 and Maturin projects to create a Python extension module written in Rust and  

This module allows to convert static (i.e. non-interactive) SVG files to
either standalone PDF files or Form XObjects that can be embedded in another
PDF file and used just like images.

## Contributing
We are looking forward to receiving your bugs and feature requests in the Issues
tab. We would also be very happy to accept PRs for bug fixes, features, or
refactorings! We'd be happy to assist you
with your PR's, so feel free to post Work in Progress PRs if marked as such.
Please be kind to the maintainers and other contributors. If you feel that there
are any problems, please feel free to reach out to us privately.

Thanks to each and every prospective contributor for the effort you (plan to)
invest in this project and for adopting it!

## License
`pysvg2pdf` is licensed under the MIT / Apache 2.0 dual license.
The same approach used for the original Rust's [svg2pdf](https://github.com/typst/svg2pdf) library.

Users and consumers of the library may choose which of those licenses they want
to apply whereas contributors have to accept that their code is in compliance
and distributed under the terms of both of these licenses.
