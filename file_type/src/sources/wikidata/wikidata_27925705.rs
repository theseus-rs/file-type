use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27925705: FileFormat = FileFormat {
    id: 27_925_705,
    puid: "wikidata/27925705",
    name: "DTED Readme file",
    extensions: &["me"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
