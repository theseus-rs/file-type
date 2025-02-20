use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48806624: FileType = FileType {
    file_format: &FileFormat {
        id: 48_806_624,
        source_type: SourceType::Wikidata,
        name: "Corel Chart",
        extensions: &["cch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
