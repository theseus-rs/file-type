use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48803275: FileType = FileType {
    file_format: &FileFormat {
        id: 48_803_275,
        source_type: SourceType::Wikidata,
        name: "Apple Sound",
        extensions: &["afc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
