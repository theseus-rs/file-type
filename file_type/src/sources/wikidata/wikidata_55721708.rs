use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55721708: FileType = FileType {
    file_format: &FileFormat {
        id: 55_721_708,
        source_type: SourceType::Wikidata,
        name: "AmiraMesh Binary Little Endian 2.1 file format",
        extensions: &["am", "amiramesh", "hx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
