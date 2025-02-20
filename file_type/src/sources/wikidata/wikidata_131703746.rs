use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131703746: FileType = FileType {
    file_format: &FileFormat {
        id: 131_703_746,
        source_type: SourceType::Wikidata,
        name: "xRage hdf file",
        extensions: &["h5rage"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
