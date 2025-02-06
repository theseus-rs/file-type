use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4192310636: FileFormat = FileFormat {
    id: 4_192_310_636,
    source_type: SourceType::Iana,
    name: "vnd.shx",
    extensions: &[],
    media_types: &["application/vnd.shx"],
    signatures: &[],
    related_formats: &[],
};
