use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7391833: FileType = FileType {
    file_format: &FileFormat {
        id: 7_391_833,
        source_type: SourceType::Wikidata,
        name: "SND",
        extensions: &["snd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
