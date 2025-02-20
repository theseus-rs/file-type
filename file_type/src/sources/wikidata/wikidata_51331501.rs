use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51331501: FileType = FileType {
    file_format: &FileFormat {
        id: 51_331_501,
        source_type: SourceType::Wikidata,
        name: "Hewlett Packard Vector Graphic Plotter File",
        extensions: &["plt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
