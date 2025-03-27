use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34739290: FileType = FileType {
    file_format: &FileFormat {
        id: 34_739_290,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 1 Person Data",
        extensions: &["fpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
