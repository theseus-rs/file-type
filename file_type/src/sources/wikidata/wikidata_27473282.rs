use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473282: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_282,
        source_type: SourceType::Wikidata,
        name: "CADRG Legend File",
        extensions: &["lgd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
