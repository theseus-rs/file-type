use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959889: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_889,
        source_type: SourceType::Wikidata,
        name: "Cubase arrangement",
        extensions: &["arr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
