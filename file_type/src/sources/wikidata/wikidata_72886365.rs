use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72886365: FileType = FileType {
    file_format: &FileFormat {
        id: 72_886_365,
        source_type: SourceType::Wikidata,
        name: "PIK",
        extensions: &["pik"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
