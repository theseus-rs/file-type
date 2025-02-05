use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2959859865: FileFormat = FileFormat {
    id: 2_959_859_865,
    source_type: SourceType::Iana,
    name: "vnd.software602.filler.form+xml",
    extensions: &[],
    media_types: &["application/vnd.software602.filler.form+xml"],
    signatures: &[],
    related_formats: &[],
};
