use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50608279: FileType = FileType {
    file_format: &FileFormat {
        id: 50_608_279,
        source_type: SourceType::Wikidata,
        name: "Phase One IIQ Raw Image",
        extensions: &["iiq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
