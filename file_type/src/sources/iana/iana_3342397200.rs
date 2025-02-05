use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3342397200: FileFormat = FileFormat {
    id: 3_342_397_200,
    source_type: SourceType::Iana,
    name: "vnd.net2phone.commcenter.command",
    extensions: &[],
    media_types: &["text/vnd.net2phone.commcenter.command"],
    signatures: &[],
    related_formats: &[],
};
