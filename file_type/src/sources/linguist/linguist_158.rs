use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_158: FileType = FileType {
    file_format: &FileFormat {
        id: 158,
        source_type: SourceType::Linguist,
        name: "Haxe",
        extensions: &["hx", "hxsl"],
        media_types: &["text/x-haxe"],
        signatures: &[],
        related_formats: &[],
    },
};
