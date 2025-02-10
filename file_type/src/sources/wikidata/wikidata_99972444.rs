use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99972444: FileType = FileType {
    file_format: &FileFormat {
        id: 99_972_444,
        source_type: SourceType::Wikidata,
        name: "Advanced Disk Catalog",
        extensions: &["adc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
