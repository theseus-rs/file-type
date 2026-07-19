use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138486211: FileType = FileType {
    file_format: &FileFormat {
        id: 138_486_211,
        source_type: SourceType::Wikidata,
        name: "FieldWorks Language Explorer FWData XML",
        extensions: &["fwdata"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
