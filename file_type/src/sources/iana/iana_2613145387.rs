use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2613145387: FileFormat = FileFormat {
    id: 2_613_145_387,
    source_type: SourceType::Iana,
    name: "hej2k",
    extensions: &[],
    media_types: &["image/hej2k"],
    internal_signatures: &[],
    related_formats: &[],
};
