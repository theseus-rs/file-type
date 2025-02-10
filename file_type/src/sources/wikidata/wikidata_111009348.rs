use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009348: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_348,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Brochure File format",
        extensions: &["bro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
