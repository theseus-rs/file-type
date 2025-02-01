use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59390889: FileFormat = FileFormat {
    id: 59_390_889,
    puid: "wikidata/59390889",
    name: "GraphPad Prism file format, version 4",
    extensions: &["pzf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
