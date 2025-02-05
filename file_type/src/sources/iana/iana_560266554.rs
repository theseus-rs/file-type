use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_560266554: FileFormat = FileFormat {
    id: 560_266_554,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
    ],
    signatures: &[],
    related_formats: &[],
};
