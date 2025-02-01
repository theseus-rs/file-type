use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121914796: FileFormat = FileFormat {
    id: 121_914_796,
    puid: "wikidata/121914796",
    name: "Microsoft Agent File",
    extensions: &["acs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
