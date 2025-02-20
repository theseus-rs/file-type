use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979406: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_406,
        source_type: SourceType::Wikidata,
        name: "QTL",
        extensions: &["qtl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
