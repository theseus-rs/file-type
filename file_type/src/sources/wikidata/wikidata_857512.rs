use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_857512: FileType = FileType {
    file_format: &FileFormat {
        id: 857_512,
        source_type: SourceType::Wikidata,
        name: "Smacker video",
        extensions: &["smk"],
        media_types: &["video/vnd.radgamettools.smacker"],
        signatures: &[],
        related_formats: &[],
    },
};
