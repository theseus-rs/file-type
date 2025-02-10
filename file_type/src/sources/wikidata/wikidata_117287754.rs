use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117287754: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_754,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot Notebook File",
        extensions: &["jnb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
