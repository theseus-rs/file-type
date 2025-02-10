use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127266247: FileType = FileType {
    file_format: &FileFormat {
        id: 127_266_247,
        source_type: SourceType::Wikidata,
        name: "Assembly file",
        extensions: &["eaf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
