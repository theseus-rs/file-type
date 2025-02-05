use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_340992391: FileFormat = FileFormat {
    id: 340_992_391,
    source_type: SourceType::Iana,
    name: "vnd.xecrets-encrypted",
    extensions: &[],
    media_types: &["application/vnd.xecrets-encrypted"],
    signatures: &[],
    related_formats: &[],
};
