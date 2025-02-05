use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1750670724: FileFormat = FileFormat {
    id: 1_750_670_724,
    source_type: SourceType::Iana,
    name: "vnd.hzn-3d-crossword",
    extensions: &[],
    media_types: &["application/vnd.hzn-3d-crossword"],
    signatures: &[],
    related_formats: &[],
};
