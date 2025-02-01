use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110238151: FileFormat = FileFormat {
    id: 110_238_151,
    puid: "wikidata/110238151",
    name: "SunRF",
    extensions: &["rf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
