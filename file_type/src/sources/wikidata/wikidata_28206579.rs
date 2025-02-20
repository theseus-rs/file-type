use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206579: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_579,
        source_type: SourceType::Wikidata,
        name: "MetaMorph Stack",
        extensions: &["stk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
