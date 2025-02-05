use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127405566: FileFormat = FileFormat {
    id: 127_405_566,
    source_type: SourceType::Wikidata,
    name: "Julia source code file",
    extensions: &["jl"],
    media_types: &["application/x-julia", "text/x-julia"],
    signatures: &[],
    related_formats: &[],
};
