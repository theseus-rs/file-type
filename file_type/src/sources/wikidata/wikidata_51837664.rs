use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51837664: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_664,
        source_type: SourceType::Wikidata,
        name: "Micrografx Designer format",
        extensions: &["dsf"],
        media_types: &["application/x-mgx-designer"],
        signatures: &[],
        related_formats: &[],
    },
};
