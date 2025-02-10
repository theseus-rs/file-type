use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50259511: FileType = FileType {
    file_format: &FileFormat {
        id: 50_259_511,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Macro-Enabled Template, version 2013",
        extensions: &["vstm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
