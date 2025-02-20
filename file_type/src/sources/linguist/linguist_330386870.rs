use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_330386870: FileType = FileType {
    file_format: &FileFormat {
        id: 330_386_870,
        source_type: SourceType::Linguist,
        name: "BQN",
        extensions: &["bqn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
