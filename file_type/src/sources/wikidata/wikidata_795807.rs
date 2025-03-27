use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_795807: FileType = FileType {
    file_format: &FileFormat {
        id: 795_807,
        source_type: SourceType::Wikidata,
        name: "BEF",
        extensions: &["bef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
