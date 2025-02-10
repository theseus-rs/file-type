use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205933: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_933,
        source_type: SourceType::Wikidata,
        name: "Doodle! uncompressed image",
        extensions: &["dd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
