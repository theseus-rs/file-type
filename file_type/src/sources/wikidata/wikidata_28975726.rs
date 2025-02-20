use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975726: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_726,
        source_type: SourceType::Wikidata,
        name: "Gaussian Cube File",
        extensions: &["cub"],
        media_types: &["chemical/x-gaussian-cube"],
        signatures: &[],
        related_formats: &[],
    },
};
