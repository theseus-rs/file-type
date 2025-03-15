use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133263690: FileType = FileType {
    file_format: &FileFormat {
        id: 133_263_690,
        source_type: SourceType::Wikidata,
        name: "Plextalk Project File (imph)",
        extensions: &["imph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
