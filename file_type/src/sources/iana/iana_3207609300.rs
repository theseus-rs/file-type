use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3207609300: FileFormat = FileFormat {
    id: 3_207_609_300,
    source_type: SourceType::Iana,
    name: "vnd.wt.stf",
    extensions: &[],
    media_types: &["application/vnd.wt.stf"],
    signatures: &[],
    related_formats: &[],
};
