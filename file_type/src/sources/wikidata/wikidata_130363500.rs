use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130363500: FileType = FileType {
    file_format: &FileFormat {
        id: 130_363_500,
        source_type: SourceType::Wikidata,
        name: "NCL file",
        extensions: &["ncl"],
        media_types: &["text/ncl"],
        signatures: &[],
        related_formats: &[],
    },
};
