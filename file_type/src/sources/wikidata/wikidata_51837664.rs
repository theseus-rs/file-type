use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
