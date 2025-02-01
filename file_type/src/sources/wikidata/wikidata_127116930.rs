use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127116930: FileFormat = FileFormat {
    id: 127_116_930,
    puid: "wikidata/127116930",
    name: "IDLSAV file",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
