use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60372734: FileFormat = FileFormat {
    id: 60_372_734,
    puid: "wikidata/60372734",
    name: "Quark Xpress Data File, version 6",
    extensions: &["qcd", "qct", "qtp", "qxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
