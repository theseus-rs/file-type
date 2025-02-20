use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3384046918: FileType = FileType {
    file_format: &FileFormat {
        id: 3_384_046_918,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
