use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110994642: FileFormat = FileFormat {
    id: 110_994_642,
    puid: "wikidata/110994642",
    name: "SnapShot Project File",
    extensions: &["ssp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
