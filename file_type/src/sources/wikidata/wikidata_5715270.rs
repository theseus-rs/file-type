use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5715270: FileType = FileType {
    file_format: &FileFormat {
        id: 5_715_270,
        source_type: SourceType::Wikidata,
        name: "Kindle File Format",
        extensions: &["amr", "azw", "azw3", "kfx", "mobi"],
        media_types: &["application/vnd.amazon.mobi8-ebook"],
        signatures: &[],
        related_formats: &[],
    },
};
