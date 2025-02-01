use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117870881: FileFormat = FileFormat {
    id: 117_870_881,
    puid: "wikidata/117870881",
    name: "TriGem SoftFax file",
    extensions: &["tri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
