use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119574681: FileType = FileType {
    file_format: &FileFormat {
        id: 119_574_681,
        source_type: SourceType::Wikidata,
        name: "Kid Pix File",
        extensions: &["kpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
