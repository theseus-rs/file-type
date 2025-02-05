use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_108293076: FileFormat = FileFormat {
    id: 108_293_076,
    source_type: SourceType::Iana,
    name: "remote-printing (OBSOLETE)",
    extensions: &[],
    media_types: &["application/remote-printing"],
    signatures: &[],
    related_formats: &[],
};
