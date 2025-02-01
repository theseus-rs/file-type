use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854544: FileFormat = FileFormat {
    id: 105_854_544,
    puid: "wikidata/105854544",
    name: "Adobe Digital Editions Adobe Content Server Message",
    extensions: &["acsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
