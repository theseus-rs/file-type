use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538988: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_988,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Last Saved Layer State",
        extensions: &["las"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
