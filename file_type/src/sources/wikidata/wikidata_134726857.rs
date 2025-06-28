use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134726857: FileType = FileType {
    file_format: &FileFormat {
        id: 134_726_857,
        source_type: SourceType::Wikidata,
        name: "CoNLL-U",
        extensions: &["conllu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
