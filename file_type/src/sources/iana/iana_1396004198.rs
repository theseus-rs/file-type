use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1396004198: FileFormat = FileFormat {
    id: 1_396_004_198,
    source_type: SourceType::Iana,
    name: "vnd.modl",
    extensions: &[],
    media_types: &["application/vnd.modl"],
    signatures: &[],
    related_formats: &[],
};
