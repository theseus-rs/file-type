use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3943569: FileFormat = FileFormat {
    id: 3_943_569,
    puid: "wikidata/3943569",
    name: "SEG-Y",
    extensions: &["segy", "sgy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
