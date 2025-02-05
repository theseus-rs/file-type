use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1984804763: FileFormat = FileFormat {
    id: 1_984_804_763,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
    ],
    signatures: &[],
    related_formats: &[],
};
