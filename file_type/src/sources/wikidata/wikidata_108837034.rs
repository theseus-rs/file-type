use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837034: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_034,
        source_type: SourceType::Wikidata,
        name: "Nero Video-CD Compilation",
        extensions: &["nrv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
