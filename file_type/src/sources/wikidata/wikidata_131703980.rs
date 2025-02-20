use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131703980: FileType = FileType {
    file_format: &FileFormat {
        id: 131_703_980,
        source_type: SourceType::Wikidata,
        name: "GE TRUCHAS file format",
        extensions: &["h5", "hdf5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
