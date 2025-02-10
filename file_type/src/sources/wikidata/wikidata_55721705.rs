use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55721705: FileType = FileType {
    file_format: &FileFormat {
        id: 55_721_705,
        source_type: SourceType::Wikidata,
        name: "AmiraMesh 3D Binary Little Endian 2.0 file format",
        extensions: &["am", "amiramesh", "hx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
