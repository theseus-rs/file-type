use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34742508: FileType = FileType {
    file_format: &FileFormat {
        id: 34_742_508,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 2 Index Data",
        extensions: &["fti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
