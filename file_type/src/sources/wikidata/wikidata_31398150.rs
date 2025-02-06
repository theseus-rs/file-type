use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_31398150: FileFormat = FileFormat {
    id: 31_398_150,
    source_type: SourceType::Wikidata,
    name: "N-Quads",
    extensions: &["nq"],
    media_types: &["application/n-quads"],
    signatures: &[],
    related_formats: &[],
};
