use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47012412: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_412,
        source_type: SourceType::Wikidata,
        name: "FoxPro Memo file format",
        extensions: &["fpt", "frt", "pjt", "vct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
