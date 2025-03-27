use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34740514: FileType = FileType {
    file_format: &FileFormat {
        id: 34_740_514,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 2 Person Data",
        extensions: &["ftd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
