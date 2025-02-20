use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_31398150: FileType = FileType {
    file_format: &FileFormat {
        id: 31_398_150,
        source_type: SourceType::Wikidata,
        name: "N-Quads",
        extensions: &["nq"],
        media_types: &["application/n-quads"],
        signatures: &[],
        related_formats: &[],
    },
};
