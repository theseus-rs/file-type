use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51789800: FileType = FileType {
    file_format: &FileFormat {
        id: 51_789_800,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio Drawing, version 5",
        extensions: &["vsd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
