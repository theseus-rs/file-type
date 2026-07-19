use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131932947: FileType = FileType {
    file_format: &FileFormat {
        id: 131_932_947,
        source_type: SourceType::Wikidata,
        name: "HDF5 File",
        extensions: &["hdf5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
