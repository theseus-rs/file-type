use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951310: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_310,
        source_type: SourceType::Wikidata,
        name: "Haskell Script File Format",
        extensions: &["hs"],
        media_types: &["text/x-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
