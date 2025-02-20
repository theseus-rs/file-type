use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49799499: FileType = FileType {
    file_format: &FileFormat {
        id: 49_799_499,
        source_type: SourceType::Wikidata,
        name: "Adobe Portable Document Catalog Index File, version 3.1",
        extensions: &["pdx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
