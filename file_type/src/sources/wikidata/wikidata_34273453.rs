use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34273453: FileType = FileType {
    file_format: &FileFormat {
        id: 34_273_453,
        source_type: SourceType::Wikidata,
        name: "Keynote Zipped",
        extensions: &["key.zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
