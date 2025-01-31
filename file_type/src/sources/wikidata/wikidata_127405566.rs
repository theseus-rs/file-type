use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127405566: FileFormat = FileFormat {
    id: 127_405_566,
    puid: "wikidata/127405566",
    name: "Julia source code file",
    extensions: &["jl", "jl"],
    media_types: &["application/x-julia", "text/x-julia"],
    internal_signatures: &[],
    related_formats: &[],
};
