use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600661: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_661,
        source_type: SourceType::Wikidata,
        name: "Disk Imploder",
        extensions: &["dex", "dmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
