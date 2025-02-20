use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206795: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_795,
        source_type: SourceType::Wikidata,
        name: "OS/2 Boot Logo",
        extensions: &["lgo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
