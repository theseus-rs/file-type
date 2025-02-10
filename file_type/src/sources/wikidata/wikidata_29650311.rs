use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650311: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_311,
        source_type: SourceType::Wikidata,
        name: "POV-Ray scene description",
        extensions: &["inc", "pov"],
        media_types: &["model/x-pov", "text/x-povray"],
        signatures: &[],
        related_formats: &[],
    },
};
