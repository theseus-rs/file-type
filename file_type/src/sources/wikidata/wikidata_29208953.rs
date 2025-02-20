use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29208953: FileType = FileType {
    file_format: &FileFormat {
        id: 29_208_953,
        source_type: SourceType::Wikidata,
        name: ".lzma File Format",
        extensions: &["lzma"],
        media_types: &["application/x-lzma"],
        signatures: &[],
        related_formats: &[],
    },
};
