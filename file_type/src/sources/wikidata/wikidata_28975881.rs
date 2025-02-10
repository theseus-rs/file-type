use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975881: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_881,
        source_type: SourceType::Wikidata,
        name: "SOLIDWORKS Part",
        extensions: &["sldprt"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
