use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61971919: FileType = FileType {
    file_format: &FileFormat {
        id: 61_971_919,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro Database Table File",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
