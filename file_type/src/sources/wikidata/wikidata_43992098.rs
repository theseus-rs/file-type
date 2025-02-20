use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_43992098: FileType = FileType {
    file_format: &FileFormat {
        id: 43_992_098,
        source_type: SourceType::Wikidata,
        name: "Extensible Metadata Platform Packet",
        extensions: &["xmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
