use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61963304: FileType = FileType {
    file_format: &FileFormat {
        id: 61_963_304,
        source_type: SourceType::Wikidata,
        name: "Microsoft Front Page Binary Tree Index",
        extensions: &["btr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
