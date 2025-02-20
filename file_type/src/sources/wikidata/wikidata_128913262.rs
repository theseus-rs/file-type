use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128913262: FileType = FileType {
    file_format: &FileFormat {
        id: 128_913_262,
        source_type: SourceType::Wikidata,
        name: "DylanLID file format",
        extensions: &["lid"],
        media_types: &["text/x-dylan-lid"],
        signatures: &[],
        related_formats: &[],
    },
};
