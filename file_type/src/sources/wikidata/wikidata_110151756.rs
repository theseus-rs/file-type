use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110151756: FileFormat = FileFormat {
    id: 110_151_756,
    puid: "wikidata/110151756",
    name: "Serif DrawPlus Drawing, version X4",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
