use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777705: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_705,
        source_type: SourceType::Wikidata,
        name: "MyHeritage Family Tree Builder",
        extensions: &["zed"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
