use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61984326: FileType = FileType {
    file_format: &FileFormat {
        id: 61_984_326,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro Project",
        extensions: &["pjx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
