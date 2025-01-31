use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110995667: FileFormat = FileFormat {
    id: 110_995_667,
    puid: "wikidata/110995667",
    name: "VideoWave Scene",
    extensions: &["scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
