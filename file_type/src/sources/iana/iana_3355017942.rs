use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3355017942: FileType = FileType {
    file_format: &FileFormat {
        id: 3_355_017_942,
        source_type: SourceType::Iana,
        name: "midi-clip",
        extensions: &[],
        media_types: &["audio/midi-clip"],
        signatures: &[],
        related_formats: &[],
    },
};
