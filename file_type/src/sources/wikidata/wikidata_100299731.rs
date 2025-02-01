use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100299731: FileFormat = FileFormat {
    id: 100_299_731,
    puid: "wikidata/100299731",
    name: "Flow Charting file format, version 6",
    extensions: &["fcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
