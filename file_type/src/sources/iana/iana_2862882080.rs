use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2862882080: FileFormat = FileFormat {
    id: 2_862_882_080,
    source_type: SourceType::Iana,
    name: "vnd.pt.mundusmundi",
    extensions: &[],
    media_types: &["application/vnd.pt.mundusmundi"],
    signatures: &[],
    related_formats: &[],
};
