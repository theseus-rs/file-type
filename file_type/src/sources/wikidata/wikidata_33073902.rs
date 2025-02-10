use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_33073902: FileType = FileType {
    file_format: &FileFormat {
        id: 33_073_902,
        source_type: SourceType::Wikidata,
        name: "Offline Web applications",
        extensions: &["appcache"],
        media_types: &["text/cache-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
