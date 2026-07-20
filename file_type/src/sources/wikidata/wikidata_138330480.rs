use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138330480: FileType = FileType {
    file_format: &FileFormat {
        id: 138_330_480,
        source_type: SourceType::Wikidata,
        name: "cpgz",
        extensions: &["cpgz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
