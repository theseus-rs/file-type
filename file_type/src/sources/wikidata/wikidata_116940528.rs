use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116940528: FileFormat = FileFormat {
    id: 116_940_528,
    puid: "wikidata/116940528",
    name: "AccessData Recovery Session",
    extensions: &["adr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
