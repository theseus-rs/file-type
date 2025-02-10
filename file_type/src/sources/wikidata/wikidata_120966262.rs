use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120966262: FileType = FileType {
    file_format: &FileFormat {
        id: 120_966_262,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 99 data",
        extensions: &["mn7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
