use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_56827137: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_137,
        source_type: SourceType::Wikidata,
        name: "Nintendo DS cartridge file",
        extensions: &["nds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
