use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_736235603: FileType = FileType {
    file_format: &FileFormat {
        id: 736_235_603,
        source_type: SourceType::Linguist,
        name: "PDDL",
        extensions: &["pddl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
