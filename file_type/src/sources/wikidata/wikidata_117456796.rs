use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117456796: FileType = FileType {
    file_format: &FileFormat {
        id: 117_456_796,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Encrypted Database File 1.0",
        extensions: &["mda", "mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
