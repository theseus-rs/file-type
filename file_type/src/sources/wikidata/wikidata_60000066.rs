use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60000066: FileType = FileType {
    file_format: &FileFormat {
        id: 60_000_066,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Owner File",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
