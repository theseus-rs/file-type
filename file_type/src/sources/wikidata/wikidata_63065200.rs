use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63065200: FileType = FileType {
    file_format: &FileFormat {
        id: 63_065_200,
        source_type: SourceType::Wikidata,
        name: "HDF4",
        extensions: &["h4", "hdf", "hdf4", "he4"],
        media_types: &["application/x-hdf", "application/x-hdf4"],
        signatures: &[],
        related_formats: &[],
    },
};
