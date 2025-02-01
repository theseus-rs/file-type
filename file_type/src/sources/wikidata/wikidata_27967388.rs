use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967388: FileFormat = FileFormat {
    id: 27_967_388,
    puid: "wikidata/27967388",
    name: "Adlib Tracker instrument",
    extensions: &["ins"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
