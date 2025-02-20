use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17175740: FileType = FileType {
    file_format: &FileFormat {
        id: 17_175_740,
        source_type: SourceType::Wikidata,
        name: "comic book archive, tar container",
        extensions: &["cbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
