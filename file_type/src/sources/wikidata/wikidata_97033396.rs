use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_97033396: FileType = FileType {
    file_format: &FileFormat {
        id: 97_033_396,
        source_type: SourceType::Wikidata,
        name: "Magick Vector Graphics",
        extensions: &["mvg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
