use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2570068954: FileFormat = FileFormat {
    id: 2_570_068_954,
    source_type: SourceType::Iana,
    name: "vnd.pg.format",
    extensions: &[],
    media_types: &["application/vnd.pg.format"],
    signatures: &[],
    related_formats: &[],
};
