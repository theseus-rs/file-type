use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3018718273: FileFormat = FileFormat {
    id: 3_018_718_273,
    source_type: SourceType::Iana,
    name: "n-quads",
    extensions: &[],
    media_types: &["application/n-quads"],
    internal_signatures: &[],
    related_formats: &[],
};
