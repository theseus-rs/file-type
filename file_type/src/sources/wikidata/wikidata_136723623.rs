use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136723623: FileType = FileType {
    file_format: &FileFormat {
        id: 136_723_623,
        source_type: SourceType::Wikidata,
        name: "Mikron Infrared Thermal Imaging Camera SIT file format",
        extensions: &["sit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
