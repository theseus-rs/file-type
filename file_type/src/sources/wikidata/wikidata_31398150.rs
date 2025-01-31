use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_31398150: FileFormat = FileFormat {
    id: 31_398_150,
    puid: "wikidata/31398150",
    name: "N-Quads",
    extensions: &["nq"],
    media_types: &["application/n-quads"],
    internal_signatures: &[],
    related_formats: &[],
};
