use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_748682461: FileFormat = FileFormat {
    id: 748_682_461,
    source_type: SourceType::Iana,
    name: "vnd.doremir.scorecloud-binary-document",
    extensions: &[],
    media_types: &["application/vnd.doremir.scorecloud-binary-document"],
    internal_signatures: &[],
    related_formats: &[],
};
