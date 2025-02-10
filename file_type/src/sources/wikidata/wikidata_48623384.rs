use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48623384: FileType = FileType {
    file_format: &FileFormat {
        id: 48_623_384,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project Export File",
        extensions: &["mpx"],
        media_types: &["application/x-project"],
        signatures: &[],
        related_formats: &[],
    },
};
