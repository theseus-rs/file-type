use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34739781: FileType = FileType {
    file_format: &FileFormat {
        id: 34_739_781,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 1 Index Data",
        extensions: &["fix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
