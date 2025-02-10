use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51800009: FileType = FileType {
    file_format: &FileFormat {
        id: 51_800_009,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Macro, version 4",
        extensions: &["xla", "xlm"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
