use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_310: FileType = FileType {
    file_format: &FileFormat {
        id: 310,
        source_type: SourceType::Linguist,
        name: "REALbasic",
        extensions: &["rbbas", "rbfrm", "rbmnu", "rbres", "rbtbar", "rbuistate"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
