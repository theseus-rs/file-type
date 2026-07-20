use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3796889: FileType = FileType {
    file_format: &FileFormat {
        id: 3_796_889,
        source_type: SourceType::Wikidata,
        name: "DV",
        extensions: &["dv"],
        media_types: &["audio/DV", "video/DV"],
        signatures: &[],
        related_formats: &[],
    },
};
