use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975873: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_873,
        source_type: SourceType::Wikidata,
        name: "OOGL LIST file",
        extensions: &["list"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
