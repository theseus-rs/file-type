use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127265031: FileType = FileType {
    file_format: &FileFormat {
        id: 127_265_031,
        source_type: SourceType::Wikidata,
        name: "ANSYS input file",
        extensions: &["cdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
