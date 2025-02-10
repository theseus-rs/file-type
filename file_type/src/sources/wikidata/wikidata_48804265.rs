use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48804265: FileType = FileType {
    file_format: &FileFormat {
        id: 48_804_265,
        source_type: SourceType::Wikidata,
        name: "Paradox Database Memo Field",
        extensions: &["dbq", "mb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
