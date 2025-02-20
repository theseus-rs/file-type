use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975912: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_912,
        source_type: SourceType::Wikidata,
        name: "XGL",
        extensions: &["xgl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
