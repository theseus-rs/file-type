use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_106729104: FileType = FileType {
    file_format: &FileFormat {
        id: 106_729_104,
        source_type: SourceType::Wikidata,
        name: "mz5",
        extensions: &["mz5"],
        media_types: &["application/x-hdf5"],
        signatures: &[],
        related_formats: &[],
    },
};
