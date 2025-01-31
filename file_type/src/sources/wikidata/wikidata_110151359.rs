use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110151359: FileFormat = FileFormat {
    id: 110_151_359,
    puid: "wikidata/110151359",
    name: "Serif DrawPlus Drawing, version X3",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
