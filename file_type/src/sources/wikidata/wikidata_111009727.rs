use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009727: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_727,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Envelope File format",
        extensions: &["env"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
