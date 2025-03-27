use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205405: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_405,
        source_type: SourceType::Wikidata,
        name: "Nikon RAW",
        extensions: &["nrw"],
        media_types: &["image/x-nikon-nrw", "image/x-raw-nikon"],
        signatures: &[],
        related_formats: &[],
    },
};
