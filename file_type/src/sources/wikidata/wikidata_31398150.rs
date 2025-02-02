use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_31398150: FileFormat = FileFormat {
    id: 31_398_150,
    source_type: SourceType::Wikidata,
    name: "N-Quads",
    extensions: &["nq"],
    media_types: &["application/n-quads"],
    internal_signatures: &[],
    related_formats: &[],
};
