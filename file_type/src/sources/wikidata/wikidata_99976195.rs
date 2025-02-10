use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99976195: FileType = FileType {
    file_format: &FileFormat {
        id: 99_976_195,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 2.0.1",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
