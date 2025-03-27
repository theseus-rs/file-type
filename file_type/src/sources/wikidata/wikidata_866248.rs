use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_866248: FileType = FileType {
    file_format: &FileFormat {
        id: 866_248,
        source_type: SourceType::Wikidata,
        name: "True Audio, version 1",
        extensions: &["tta"],
        media_types: &["audio/tta", "audio/x-tta"],
        signatures: &[],
        related_formats: &[],
    },
};
