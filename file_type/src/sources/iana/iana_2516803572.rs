use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2516803572: FileFormat = FileFormat {
    id: 2_516_803_572,
    source_type: SourceType::Iana,
    name: "vnd.etsi.asic-s+zip",
    extensions: &[],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
