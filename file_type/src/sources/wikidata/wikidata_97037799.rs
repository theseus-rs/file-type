use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_97037799: FileType = FileType {
    file_format: &FileFormat {
        id: 97_037_799,
        source_type: SourceType::Wikidata,
        name: "HP Page Control Language",
        extensions: &["pcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
