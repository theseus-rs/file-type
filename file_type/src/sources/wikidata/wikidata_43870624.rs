use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43870624: FileType = FileType {
    file_format: &FileFormat {
        id: 43_870_624,
        source_type: SourceType::Wikidata,
        name: "PCX, version 5",
        extensions: &["pcc", "pcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
