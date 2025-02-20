use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121133216: FileType = FileType {
    file_format: &FileFormat {
        id: 121_133_216,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for Windows BitMap",
        extensions: &["bmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
