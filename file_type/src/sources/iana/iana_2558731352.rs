use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2558731352: FileType = FileType {
    file_format: &FileFormat {
        id: 2_558_731_352,
        source_type: SourceType::Iana,
        name: "mizar",
        extensions: &[],
        media_types: &["text/mizar"],
        signatures: &[],
        related_formats: &[],
    },
};
