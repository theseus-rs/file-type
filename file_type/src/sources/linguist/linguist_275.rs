use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_275: FileFormat = FileFormat {
    id: 275,
    source_type: SourceType::Linguist,
    name: "POV-Ray SDL",
    extensions: &["inc", "pov"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
