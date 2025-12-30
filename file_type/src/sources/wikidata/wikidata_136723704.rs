use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136723704: FileType = FileType {
    file_format: &FileFormat {
        id: 136_723_704,
        source_type: SourceType::Wikidata,
        name: "AutoSketch file format",
        extensions: &["skd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
