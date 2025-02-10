use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27473250: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_250,
        source_type: SourceType::Wikidata,
        name: "Raster Product Format Table of Contents File",
        extensions: &["toc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
