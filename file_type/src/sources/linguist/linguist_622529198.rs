use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_622529198: FileType = FileType {
    file_format: &FileFormat {
        id: 622_529_198,
        source_type: SourceType::Linguist,
        name: "Faust",
        extensions: &["dsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
