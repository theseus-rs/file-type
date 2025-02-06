use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_211208613: FileFormat = FileFormat {
    id: 211_208_613,
    source_type: SourceType::Iana,
    name: "xcon-conference-info+xml",
    extensions: &[],
    media_types: &["application/xcon-conference-info+xml"],
    signatures: &[],
    related_formats: &[],
};
