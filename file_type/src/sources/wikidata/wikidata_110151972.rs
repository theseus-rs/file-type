use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110151972: FileFormat = FileFormat {
    id: 110_151_972,
    puid: "wikidata/110151972",
    name: "Serif DrawPlus Drawing, version X5",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
