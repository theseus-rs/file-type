use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60371646: FileFormat = FileFormat {
    id: 60_371_646,
    puid: "wikidata/60371646",
    name: "Quark Xpress Data File, version 10",
    extensions: &["qcd", "qct", "qtp", "qxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
