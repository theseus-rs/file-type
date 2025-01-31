use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96035181: FileFormat = FileFormat {
    id: 96_035_181,
    puid: "wikidata/96035181",
    name: "LEDA",
    extensions: &["gw", "lgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
