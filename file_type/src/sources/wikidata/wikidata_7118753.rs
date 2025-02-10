use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7118753: FileType = FileType {
    file_format: &FileFormat {
        id: 7_118_753,
        source_type: SourceType::Wikidata,
        name: "PDB",
        extensions: &["pdb"],
        media_types: &["application/vnd.palm"],
        signatures: &[],
        related_formats: &[],
    },
};
