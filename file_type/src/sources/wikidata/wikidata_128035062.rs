use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128035062: FileType = FileType {
    file_format: &FileFormat {
        id: 128_035_062,
        source_type: SourceType::Wikidata,
        name: "Protein Data Bank File 3.3",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
