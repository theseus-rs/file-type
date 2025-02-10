use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272528: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_528,
        source_type: SourceType::Wikidata,
        name: "Everest embedded bank file",
        extensions: &["emb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
