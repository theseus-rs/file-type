use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51789800: FileFormat = FileFormat {
    id: 51_789_800,
    puid: "wikidata/51789800",
    name: "Microsoft Visio Drawing, version 5",
    extensions: &["vsd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
