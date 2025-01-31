use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17149857: FileFormat = FileFormat {
    id: 17_149_857,
    puid: "wikidata/17149857",
    name: "zone file",
    extensions: &["zone"],
    media_types: &["text/dns"],
    internal_signatures: &[],
    related_formats: &[],
};
