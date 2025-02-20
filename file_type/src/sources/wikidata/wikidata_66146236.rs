use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66146236: FileType = FileType {
    file_format: &FileFormat {
        id: 66_146_236,
        source_type: SourceType::Wikidata,
        name: "InfoPath Form Definition File",
        extensions: &["xsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
