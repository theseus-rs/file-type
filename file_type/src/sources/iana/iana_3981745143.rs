use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3981745143: FileFormat = FileFormat {
    id: 3_981_745_143,
    source_type: SourceType::Iana,
    name: "vnd.webturbo",
    extensions: &[],
    media_types: &["application/vnd.webturbo"],
    signatures: &[],
    related_formats: &[],
};
