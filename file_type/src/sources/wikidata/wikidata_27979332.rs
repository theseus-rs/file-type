use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979332: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_332,
        source_type: SourceType::Wikidata,
        name: "Age of Mythology BAR format",
        extensions: &["bar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
