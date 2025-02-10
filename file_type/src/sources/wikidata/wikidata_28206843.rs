use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206843: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_843,
        source_type: SourceType::Wikidata,
        name: "Palm Database ImageViewer",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
