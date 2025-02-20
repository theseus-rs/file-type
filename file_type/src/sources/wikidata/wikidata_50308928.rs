use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308928: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_928,
        source_type: SourceType::Wikidata,
        name: "Final Draft 5-7 Document",
        extensions: &["fdr"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
