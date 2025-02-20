use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009607: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_607,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Letterhead File format",
        extensions: &["let"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
