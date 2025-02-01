use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_743275: FileFormat = FileFormat {
    id: 743_275,
    puid: "wikidata/743275",
    name: "T.38",
    extensions: &["t38", "t38"],
    media_types: &["audio/t38", "image/t38"],
    internal_signatures: &[],
    related_formats: &[],
};
