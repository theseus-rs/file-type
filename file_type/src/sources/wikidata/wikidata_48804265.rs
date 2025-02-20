use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
