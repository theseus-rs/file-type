use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205661: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_661,
        source_type: SourceType::Wikidata,
        name: "Acorn Sprite",
        extensions: &["acorn"],
        media_types: &["image/x-riscos-sprite"],
        signatures: &[],
        related_formats: &[],
    },
};
