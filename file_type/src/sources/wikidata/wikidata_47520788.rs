use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47520788: FileFormat = FileFormat {
    id: 47_520_788,
    puid: "wikidata/47520788",
    name: "Serif PagePlus Publication file format, version 10",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
