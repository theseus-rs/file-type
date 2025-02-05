use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_96918703: FileFormat = FileFormat {
    id: 96_918_703,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.iufp",
    extensions: &[],
    media_types: &["audio/vnd.3gpp.iufp"],
    signatures: &[],
    related_formats: &[],
};
