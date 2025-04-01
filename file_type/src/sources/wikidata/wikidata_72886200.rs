use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72886200: FileType = FileType {
    file_format: &FileFormat {
        id: 72_886_200,
        source_type: SourceType::Wikidata,
        name: "Free Universal Image Format",
        extensions: &["fuif"],
        media_types: &["image/x-fuif"],
        signatures: &[],
        related_formats: &[],
    },
};
