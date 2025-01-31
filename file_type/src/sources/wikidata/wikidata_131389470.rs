use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131389470: FileFormat = FileFormat {
    id: 131_389_470,
    puid: "wikidata/131389470",
    name: "Varnish Configuration Language file format",
    extensions: &["vcl"],
    media_types: &["text/x-vclsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
