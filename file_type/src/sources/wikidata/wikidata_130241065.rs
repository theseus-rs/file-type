use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130241065: FileType = FileType {
    file_format: &FileFormat {
        id: 130_241_065,
        source_type: SourceType::Wikidata,
        name: "Literate Idris source code file",
        extensions: &["lidr"],
        media_types: &["text/x-literate-idris"],
        signatures: &[],
        related_formats: &[],
    },
};
