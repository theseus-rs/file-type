use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96271500: FileType = FileType {
    file_format: &FileFormat {
        id: 96_271_500,
        source_type: SourceType::Wikidata,
        name: "FlagMaker file format",
        extensions: &["flag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
