use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122073134: FileType = FileType {
    file_format: &FileFormat {
        id: 122_073_134,
        source_type: SourceType::Wikidata,
        name: "MidiScan File",
        extensions: &["mnd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
