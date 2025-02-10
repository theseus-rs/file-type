use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650305: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_305,
        source_type: SourceType::Wikidata,
        name: "PSI-MI XML",
        extensions: &["dag", "def"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
