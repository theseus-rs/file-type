use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28346237: FileFormat = FileFormat {
    id: 28_346_237,
    puid: "wikidata/28346237",
    name: "TTDDD",
    extensions: &["ttd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
