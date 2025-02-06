use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3838445211: FileFormat = FileFormat {
    id: 3_838_445_211,
    source_type: SourceType::Iana,
    name: "vnd.ipld.raw",
    extensions: &[],
    media_types: &["application/vnd.ipld.raw"],
    signatures: &[],
    related_formats: &[],
};
