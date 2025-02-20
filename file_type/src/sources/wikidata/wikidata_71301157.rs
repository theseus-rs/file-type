use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71301157: FileType = FileType {
    file_format: &FileFormat {
        id: 71_301_157,
        source_type: SourceType::Wikidata,
        name: "WHIP! DWF Format",
        extensions: &["dwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
