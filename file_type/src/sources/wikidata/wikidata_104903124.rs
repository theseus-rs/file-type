use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_104903124: FileType = FileType {
    file_format: &FileFormat {
        id: 104_903_124,
        source_type: SourceType::Wikidata,
        name: "Web Archive Collection Zipped",
        extensions: &["wacz"],
        media_types: &["application/x-wacz"],
        signatures: &[],
        related_formats: &[],
    },
};
