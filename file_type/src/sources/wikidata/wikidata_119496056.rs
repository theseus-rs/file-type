use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119496056: FileFormat = FileFormat {
    id: 119_496_056,
    puid: "wikidata/119496056",
    name: "IBM IOCA Raw",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
