use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122074310: FileType = FileType {
    file_format: &FileFormat {
        id: 122_074_310,
        source_type: SourceType::Wikidata,
        name: "SmartScore File",
        extensions: &["fin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
