use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28346117: FileType = FileType {
    file_format: &FileFormat {
        id: 28_346_117,
        source_type: SourceType::Wikidata,
        name: "Adobe Cross Domain Policy File",
        extensions: &[],
        media_types: &["text/x-cross-domain-policy"],
        signatures: &[],
        related_formats: &[],
    },
};
