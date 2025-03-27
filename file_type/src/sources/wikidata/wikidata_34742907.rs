use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34742907: FileType = FileType {
    file_format: &FileFormat {
        id: 34_742_907,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 3 Person Data",
        extensions: &["f3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
