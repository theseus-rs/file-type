use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110151085: FileFormat = FileFormat {
    id: 110_151_085,
    puid: "wikidata/110151085",
    name: "Serif DrawPlus Drawing, version X2",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
