use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119823553: FileType = FileType {
    file_format: &FileFormat {
        id: 119_823_553,
        source_type: SourceType::Wikidata,
        name: "BNTX",
        extensions: &["bntx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
