use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49798739: FileType = FileType {
    file_format: &FileFormat {
        id: 49_798_739,
        source_type: SourceType::Wikidata,
        name: "Adobe Portable Document Catalog Index File, version 3",
        extensions: &["pdx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
