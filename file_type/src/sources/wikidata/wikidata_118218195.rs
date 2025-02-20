use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118218195: FileType = FileType {
    file_format: &FileFormat {
        id: 118_218_195,
        source_type: SourceType::Wikidata,
        name: "FotoFinish Layout Template",
        extensions: &["fdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
