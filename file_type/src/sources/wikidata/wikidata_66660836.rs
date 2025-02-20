use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66660836: FileType = FileType {
    file_format: &FileFormat {
        id: 66_660_836,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Sequence",
        extensions: &["psq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
