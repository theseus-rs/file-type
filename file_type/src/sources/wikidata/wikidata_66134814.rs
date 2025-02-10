use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66134814: FileType = FileType {
    file_format: &FileFormat {
        id: 66_134_814,
        source_type: SourceType::Wikidata,
        name: "Access Add-in file format",
        extensions: &["mda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
