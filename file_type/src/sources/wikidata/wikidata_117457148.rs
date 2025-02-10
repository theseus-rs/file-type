use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117457148: FileType = FileType {
    file_format: &FileFormat {
        id: 117_457_148,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Encrypted Database File 2.0",
        extensions: &["mda", "mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
