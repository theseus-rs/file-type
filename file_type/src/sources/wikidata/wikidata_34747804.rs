use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34747804: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_804,
        source_type: SourceType::Wikidata,
        name: "Supaplex Level format",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
