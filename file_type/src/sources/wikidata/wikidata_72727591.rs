use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72727591: FileType = FileType {
    file_format: &FileFormat {
        id: 72_727_591,
        source_type: SourceType::Wikidata,
        name: "Juno address book",
        extensions: &["nv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
