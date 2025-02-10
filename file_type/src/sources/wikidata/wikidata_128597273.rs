use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128597273: FileType = FileType {
    file_format: &FileFormat {
        id: 128_597_273,
        source_type: SourceType::Wikidata,
        name: "Ampl file",
        extensions: &["run"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
