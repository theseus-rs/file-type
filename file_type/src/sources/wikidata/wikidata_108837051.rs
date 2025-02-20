use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837051: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_051,
        source_type: SourceType::Wikidata,
        name: "Nero UDF CD-ROM Compilation",
        extensions: &["nru"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
