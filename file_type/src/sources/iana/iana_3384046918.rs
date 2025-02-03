use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3384046918: FileFormat = FileFormat {
    id: 3_384_046_918,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
    extensions: &[],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
