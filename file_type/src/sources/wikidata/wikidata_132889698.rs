use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132889698: FileType = FileType {
    file_format: &FileFormat {
        id: 132_889_698,
        source_type: SourceType::Wikidata,
        name: "RawACF file",
        extensions: &["rawacf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
