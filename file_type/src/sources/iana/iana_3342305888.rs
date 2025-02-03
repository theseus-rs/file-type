use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3342305888: FileFormat = FileFormat {
    id: 3_342_305_888,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
