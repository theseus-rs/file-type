use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_15837131: FileType = FileType {
    file_format: &FileFormat {
        id: 15_837_131,
        source_type: SourceType::Wikidata,
        name: "Nuppelvideo",
        extensions: &["nuv"],
        media_types: &["video/nuppelVideo"],
        signatures: &[],
        related_formats: &[],
    },
};
