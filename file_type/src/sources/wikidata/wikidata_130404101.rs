use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130404101: FileType = FileType {
    file_format: &FileFormat {
        id: 130_404_101,
        source_type: SourceType::Wikidata,
        name: "Opa file format",
        extensions: &["opa"],
        media_types: &["text/x-opa"],
        signatures: &[],
        related_formats: &[],
    },
};
