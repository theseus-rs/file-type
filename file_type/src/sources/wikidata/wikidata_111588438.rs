use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111588438: FileType = FileType {
    file_format: &FileFormat {
        id: 111_588_438,
        source_type: SourceType::Wikidata,
        name: "Roxio Label Creator Project File, versions 6-7",
        extensions: &["jwl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
