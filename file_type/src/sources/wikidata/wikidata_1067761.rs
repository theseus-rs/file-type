use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1067761: FileType = FileType {
    file_format: &FileFormat {
        id: 1_067_761,
        source_type: SourceType::Wikidata,
        name: "Windows Media Audio 9 Lossless",
        extensions: &["wma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
