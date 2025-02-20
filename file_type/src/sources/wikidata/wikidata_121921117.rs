use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121921117: FileType = FileType {
    file_format: &FileFormat {
        id: 121_921_117,
        source_type: SourceType::Wikidata,
        name: "Ptex File Format",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
