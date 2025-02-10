use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009753: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_753,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Web Page File format",
        extensions: &["web"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
