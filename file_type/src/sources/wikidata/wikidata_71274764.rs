use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71274764: FileType = FileType {
    file_format: &FileFormat {
        id: 71_274_764,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Template",
        extensions: &["cdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
