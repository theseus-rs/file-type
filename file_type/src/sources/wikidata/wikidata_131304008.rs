use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131304008: FileType = FileType {
    file_format: &FileFormat {
        id: 131_304_008,
        source_type: SourceType::Wikidata,
        name: "Riverbed Stingray Traffic Manager file format",
        extensions: &["rts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
