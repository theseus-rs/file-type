use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51717333: FileType = FileType {
    file_format: &FileFormat {
        id: 51_717_333,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint Presentation, version 95",
        extensions: &["ppt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
