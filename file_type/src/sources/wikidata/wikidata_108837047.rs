use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837047: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_047,
        source_type: SourceType::Wikidata,
        name: "Nero CD-ROM Boot Compilation",
        extensions: &["nrb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
