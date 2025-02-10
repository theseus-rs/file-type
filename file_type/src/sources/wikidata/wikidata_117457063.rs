use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117457063: FileType = FileType {
    file_format: &FileFormat {
        id: 117_457_063,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Encrypted Database File 1.1",
        extensions: &["mda", "mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
