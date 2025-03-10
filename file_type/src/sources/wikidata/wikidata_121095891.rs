use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121095891: FileType = FileType {
    file_format: &FileFormat {
        id: 121_095_891,
        source_type: SourceType::Wikidata,
        name: "HDF-EOS",
        extensions: &[],
        media_types: &["application/x-hdf", "application/x-hdf4"],
        signatures: &[],
        related_formats: &[],
    },
};
