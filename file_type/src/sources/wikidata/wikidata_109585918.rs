use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109585918: FileType = FileType {
    file_format: &FileFormat {
        id: 109_585_918,
        source_type: SourceType::Wikidata,
        name: "Painter framestack file format",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
