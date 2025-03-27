use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131363833: FileType = FileType {
    file_format: &FileFormat {
        id: 131_363_833,
        source_type: SourceType::Wikidata,
        name: "Open Score Format",
        extensions: &["osf"],
        media_types: &["application/vnd.yamaha.openscoreformat"],
        signatures: &[],
        related_formats: &[],
    },
};
