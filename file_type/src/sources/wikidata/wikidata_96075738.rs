use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96075738: FileType = FileType {
    file_format: &FileFormat {
        id: 96_075_738,
        source_type: SourceType::Wikidata,
        name: "Pajek format",
        extensions: &["net"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
