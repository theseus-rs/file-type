use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286754: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_754,
        source_type: SourceType::Wikidata,
        name: "Clipper memory variables file",
        extensions: &["mem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
