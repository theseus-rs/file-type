use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51334664: FileType = FileType {
    file_format: &FileFormat {
        id: 51_334_664,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint Presentation, version 4",
        extensions: &["ppt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
