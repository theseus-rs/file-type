use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110477513: FileType = FileType {
    file_format: &FileFormat {
        id: 110_477_513,
        source_type: SourceType::Wikidata,
        name: "RSX",
        extensions: &["rsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
