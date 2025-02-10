use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
