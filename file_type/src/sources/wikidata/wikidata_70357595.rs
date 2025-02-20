use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_70357595: FileType = FileType {
    file_format: &FileFormat {
        id: 70_357_595,
        source_type: SourceType::Wikidata,
        name: "Jupyter notebook file",
        extensions: &["ipynb"],
        media_types: &["application/x-ipynb+json"],
        signatures: &[],
        related_formats: &[],
    },
};
