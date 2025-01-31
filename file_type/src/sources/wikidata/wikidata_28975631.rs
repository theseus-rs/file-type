use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975631: FileFormat = FileFormat {
    id: 28_975_631,
    puid: "wikidata/28975631",
    name: "Moray User Defined Object",
    extensions: &["udo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
