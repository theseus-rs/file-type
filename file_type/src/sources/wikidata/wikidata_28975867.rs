use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975867: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_867,
        source_type: SourceType::Wikidata,
        name: "OOGL SKEL file",
        extensions: &["skel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
