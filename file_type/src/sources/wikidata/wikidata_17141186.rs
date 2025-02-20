use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17141186: FileType = FileType {
    file_format: &FileFormat {
        id: 17_141_186,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help 2",
        extensions: &["hxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
