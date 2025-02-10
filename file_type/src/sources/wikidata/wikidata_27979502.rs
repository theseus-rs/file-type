use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979502: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_502,
        source_type: SourceType::Wikidata,
        name: "DNG camera profile",
        extensions: &["dcp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
