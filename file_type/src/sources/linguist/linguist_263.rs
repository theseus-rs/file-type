use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_263: FileFormat = FileFormat {
    id: 263,
    source_type: SourceType::Linguist,
    name: "OpenCL",
    extensions: &["cl", "opencl"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
