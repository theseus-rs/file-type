use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205390: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_390,
        source_type: SourceType::Wikidata,
        name: "Minolta Digital Camera",
        extensions: &["mdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
