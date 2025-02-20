use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771322: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_322,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database file format (backup file)",
        extensions: &["bdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
