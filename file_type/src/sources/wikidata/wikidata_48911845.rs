use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48911845: FileType = FileType {
    file_format: &FileFormat {
        id: 48_911_845,
        source_type: SourceType::Wikidata,
        name: "Hewlett Packard AdvanceWrite Text File",
        extensions: &["aw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
