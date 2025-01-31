use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111588712: FileFormat = FileFormat {
    id: 111_588_712,
    puid: "wikidata/111588712",
    name: "Roxio Label Creator Project File, versions 8-11",
    extensions: &["jwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
