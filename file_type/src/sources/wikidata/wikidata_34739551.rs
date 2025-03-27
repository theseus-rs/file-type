use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34739551: FileType = FileType {
    file_format: &FileFormat {
        id: 34_739_551,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 1 Marriage Data",
        extensions: &["fmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
