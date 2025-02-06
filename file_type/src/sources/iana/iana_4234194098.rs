use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4234194098: FileFormat = FileFormat {
    id: 4_234_194_098,
    source_type: SourceType::Iana,
    name: "vnd.emclient.accessrequest+xml",
    extensions: &[],
    media_types: &["application/vnd.emclient.accessrequest+xml"],
    signatures: &[],
    related_formats: &[],
};
