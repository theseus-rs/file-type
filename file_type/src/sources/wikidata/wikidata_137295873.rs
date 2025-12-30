use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137295873: FileType = FileType {
    file_format: &FileFormat {
        id: 137_295_873,
        source_type: SourceType::Wikidata,
        name: "Universal Data Link",
        extensions: &["udl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
