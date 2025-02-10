use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169866: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_866,
        source_type: SourceType::Wikidata,
        name: "Lotus Notes User ID File",
        extensions: &["id"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
