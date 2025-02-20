use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110086227: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_227,
        source_type: SourceType::Wikidata,
        name: "NTI JewelCase Maker format",
        extensions: &["jwc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
