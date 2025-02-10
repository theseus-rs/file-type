use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87984761: FileType = FileType {
    file_format: &FileFormat {
        id: 87_984_761,
        source_type: SourceType::Wikidata,
        name: "CorelCHART Document 3-4",
        extensions: &["cch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
