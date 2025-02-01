use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28955450: FileFormat = FileFormat {
    id: 28_955_450,
    puid: "wikidata/28955450",
    name: "GenBank file format",
    extensions: &["gb", "gbk", "gen"],
    media_types: &[
        "chemical/x-genbank",
        "chemical/x-genbank",
        "chemical/x-genbank",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
