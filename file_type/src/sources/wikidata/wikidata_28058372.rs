use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28058372: FileFormat = FileFormat {
    id: 28_058_372,
    puid: "wikidata/28058372",
    name: "IFF-FAXX",
    extensions: &["faxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
