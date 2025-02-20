use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009835: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_835,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Post Card File format",
        extensions: &["pcr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
