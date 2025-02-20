use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837028: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_028,
        source_type: SourceType::Wikidata,
        name: "Nero CD EXTRA Compilation",
        extensions: &["nrm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
