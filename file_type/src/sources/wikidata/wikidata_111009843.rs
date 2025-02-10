use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009843: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_843,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Sticker File format",
        extensions: &["sti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
