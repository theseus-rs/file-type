use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110977953: FileType = FileType {
    file_format: &FileFormat {
        id: 110_977_953,
        source_type: SourceType::Wikidata,
        name: "AutoDesk 16-bit Animation File",
        extensions: &["flx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
