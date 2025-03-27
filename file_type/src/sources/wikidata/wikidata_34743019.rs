use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34743019: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_019,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 3 Marriage Data",
        extensions: &["f3m"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
