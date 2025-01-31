use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_82730668: FileFormat = FileFormat {
    id: 82_730_668,
    puid: "wikidata/82730668",
    name: "Microsoft Entourage Archive",
    extensions: &["rge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
