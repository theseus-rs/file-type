use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966952: FileFormat = FileFormat {
    id: 27_966_952,
    puid: "wikidata/27966952",
    name: "SSF",
    extensions: &["minissf", "ssf", "ssflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
