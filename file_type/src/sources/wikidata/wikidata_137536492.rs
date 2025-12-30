use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137536492: FileType = FileType {
    file_format: &FileFormat {
        id: 137_536_492,
        source_type: SourceType::Wikidata,
        name: "Stellarium script file",
        extensions: &["ssc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
