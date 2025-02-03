use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1848133025: FileFormat = FileFormat {
    id: 1_848_133_025,
    source_type: SourceType::Iana,
    name: "x3d-vrml",
    extensions: &[],
    media_types: &["model/x3d-vrml"],
    internal_signatures: &[],
    related_formats: &[],
};
