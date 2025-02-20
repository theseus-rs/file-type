use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1682527214: FileType = FileType {
    file_format: &FileFormat {
        id: 1_682_527_214,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
