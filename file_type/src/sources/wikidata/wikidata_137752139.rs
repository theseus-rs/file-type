use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137752139: FileType = FileType {
    file_format: &FileFormat {
        id: 137_752_139,
        source_type: SourceType::Wikidata,
        name: "Hybrix PNG ROM cartridge format",
        extensions: &["png"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
