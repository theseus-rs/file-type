use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73514919: FileType = FileType {
    file_format: &FileFormat {
        id: 73_514_919,
        source_type: SourceType::Wikidata,
        name: "Package Resource Index",
        extensions: &["pri"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
