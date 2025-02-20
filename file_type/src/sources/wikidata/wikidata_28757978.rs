use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757978: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_978,
        source_type: SourceType::Wikidata,
        name: "Precompiled Windows Setup Information File",
        extensions: &["pnf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
