use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127405566: FileType = FileType {
    file_format: &FileFormat {
        id: 127_405_566,
        source_type: SourceType::Wikidata,
        name: "Julia source code file",
        extensions: &["jl"],
        media_types: &["application/x-julia", "text/x-julia"],
        signatures: &[],
        related_formats: &[],
    },
};
