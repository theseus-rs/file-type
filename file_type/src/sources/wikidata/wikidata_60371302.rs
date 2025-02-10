use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60371302: FileType = FileType {
    file_format: &FileFormat {
        id: 60_371_302,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint Macro-Enabled Show",
        extensions: &["ppsm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
