use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137828072: FileType = FileType {
    file_format: &FileFormat {
        id: 137_828_072,
        source_type: SourceType::Wikidata,
        name: "Mac OSX strings file",
        extensions: &["strings"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
