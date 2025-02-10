use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121815925: FileType = FileType {
    file_format: &FileFormat {
        id: 121_815_925,
        source_type: SourceType::Wikidata,
        name: "GST Art File 2",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
