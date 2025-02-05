use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1724910760: FileFormat = FileFormat {
    id: 1_724_910_760,
    source_type: SourceType::Iana,
    name: "vnd.powerbuilder75-s",
    extensions: &[],
    media_types: &["application/vnd.powerbuilder75-s"],
    signatures: &[],
    related_formats: &[],
};
