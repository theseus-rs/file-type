use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4263354093: FileFormat = FileFormat {
    id: 4_263_354_093,
    source_type: SourceType::Iana,
    name: "jxsv",
    extensions: &[],
    media_types: &["video/jxsv"],
    internal_signatures: &[],
    related_formats: &[],
};
