use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_95994804: FileType = FileType {
    file_format: &FileFormat {
        id: 95_994_804,
        source_type: SourceType::Wikidata,
        name: "Spatial Data Transfer Standard format",
        extensions: &["ddf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
