use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2551238913: FileFormat = FileFormat {
    id: 2_551_238_913,
    source_type: SourceType::Iana,
    name: "prs.btif",
    extensions: &[],
    media_types: &["image/prs.btif"],
    signatures: &[],
    related_formats: &[],
};
