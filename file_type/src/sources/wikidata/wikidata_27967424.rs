use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967424: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_424,
        source_type: SourceType::Wikidata,
        name: "Amazon downloader file",
        extensions: &["amz"],
        media_types: &["audio/x-amzxml"],
        signatures: &[],
        related_formats: &[],
    },
};
