use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_124: FileFormat = FileFormat {
    id: 124,
    source_type: SourceType::Linguist,
    name: "GLSL",
    extensions: &[
        "fp", "frag", "frg", "fs", "fsh", "fshader", "geo", "geom", "glsl", "glslf", "glslv", "gs",
        "gshader", "rchit", "rmiss", "shader", "tesc", "tese", "vert", "vrx", "vs", "vsh",
        "vshader",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
