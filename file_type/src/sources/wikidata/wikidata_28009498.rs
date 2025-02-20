use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28009498: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_498,
        source_type: SourceType::Wikidata,
        name: "Zj-Stream",
        extensions: &["prn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
