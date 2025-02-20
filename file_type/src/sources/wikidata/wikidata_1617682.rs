use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1617682: FileType = FileType {
    file_format: &FileFormat {
        id: 1_617_682,
        source_type: SourceType::Wikidata,
        name: "DVD BUP File",
        extensions: &["bup"],
        media_types: &["video/dvd"],
        signatures: &[],
        related_formats: &[],
    },
};
