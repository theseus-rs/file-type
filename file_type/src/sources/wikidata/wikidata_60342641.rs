use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60342641: FileType = FileType {
    file_format: &FileFormat {
        id: 60_342_641,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Macro-Enabled Template",
        extensions: &["xltm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
