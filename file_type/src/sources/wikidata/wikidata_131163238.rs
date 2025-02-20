use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131163238: FileType = FileType {
    file_format: &FileFormat {
        id: 131_163_238,
        source_type: SourceType::Wikidata,
        name: "Stan model file",
        extensions: &["stan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
