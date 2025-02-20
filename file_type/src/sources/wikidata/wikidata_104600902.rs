use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_104600902: FileType = FileType {
    file_format: &FileFormat {
        id: 104_600_902,
        source_type: SourceType::Wikidata,
        name: "KDB",
        extensions: &["kdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
