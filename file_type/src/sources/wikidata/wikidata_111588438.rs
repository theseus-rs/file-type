use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111588438: FileFormat = FileFormat {
    id: 111_588_438,
    puid: "wikidata/111588438",
    name: "Roxio Label Creator Project File, versions 6-7",
    extensions: &["jwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
