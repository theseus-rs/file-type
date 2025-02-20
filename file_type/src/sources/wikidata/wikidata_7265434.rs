use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7265434: FileType = FileType {
    file_format: &FileFormat {
        id: 7_265_434,
        source_type: SourceType::Wikidata,
        name: "Quicken Financial Exchange",
        extensions: &["qfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
