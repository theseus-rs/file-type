use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113652622: FileType = FileType {
    file_format: &FileFormat {
        id: 113_652_622,
        source_type: SourceType::Wikidata,
        name: "G3 1-D encoded FAX file format",
        extensions: &["fax"],
        media_types: &["image/g3fax"],
        signatures: &[],
        related_formats: &[],
    },
};
