use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55721671: FileType = FileType {
    file_format: &FileFormat {
        id: 55_721_671,
        source_type: SourceType::Wikidata,
        name: "AmiraMesh 3D ASCII 2.0 file format",
        extensions: &["am", "amiramesh", "hx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
