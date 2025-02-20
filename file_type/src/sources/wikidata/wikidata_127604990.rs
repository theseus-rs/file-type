use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127604990: FileType = FileType {
    file_format: &FileFormat {
        id: 127_604_990,
        source_type: SourceType::Wikidata,
        name: "Awk script",
        extensions: &["awk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
