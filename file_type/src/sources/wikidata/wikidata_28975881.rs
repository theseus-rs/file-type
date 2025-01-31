use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975881: FileFormat = FileFormat {
    id: 28_975_881,
    puid: "wikidata/28975881",
    name: "SOLIDWORKS Part",
    extensions: &["sldprt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
