use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125809176: FileType = FileType {
    file_format: &FileFormat {
        id: 125_809_176,
        source_type: SourceType::Wikidata,
        name: "Bzip2 Compressed Archive",
        extensions: &["bzip2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
