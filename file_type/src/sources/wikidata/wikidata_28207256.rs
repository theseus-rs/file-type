use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207256: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_256,
        source_type: SourceType::Wikidata,
        name: "ScreenShot Hack PDB",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
