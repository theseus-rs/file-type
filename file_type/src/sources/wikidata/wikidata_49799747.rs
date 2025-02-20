use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49799747: FileType = FileType {
    file_format: &FileFormat {
        id: 49_799_747,
        source_type: SourceType::Wikidata,
        name: "Adobe Portable Document Catalog Index File, version 3.2",
        extensions: &["pdx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
