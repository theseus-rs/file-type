use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136093233: FileType = FileType {
    file_format: &FileFormat {
        id: 136_093_233,
        source_type: SourceType::Wikidata,
        name: "Storyist file format",
        extensions: &["story"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
