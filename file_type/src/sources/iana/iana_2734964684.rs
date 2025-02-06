use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2734964684: FileFormat = FileFormat {
    id: 2_734_964_684,
    source_type: SourceType::Iana,
    name: "vnd.ntt-local.file-transfer",
    extensions: &[],
    media_types: &["application/vnd.ntt-local.file-transfer"],
    signatures: &[],
    related_formats: &[],
};
