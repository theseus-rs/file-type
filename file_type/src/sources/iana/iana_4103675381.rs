use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4103675381: FileFormat = FileFormat {
    id: 4_103_675_381,
    source_type: SourceType::Iana,
    name: "vnd.epson.salt",
    extensions: &[],
    media_types: &["application/vnd.epson.salt"],
    internal_signatures: &[],
    related_formats: &[],
};
