use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131703407: FileType = FileType {
    file_format: &FileFormat {
        id: 131_703_407,
        source_type: SourceType::Wikidata,
        name: "CONVERGE CFD file format",
        extensions: &["h5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
