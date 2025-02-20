use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205580: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_580,
        source_type: SourceType::Wikidata,
        name: "OS/2 Icon",
        extensions: &["ico"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
