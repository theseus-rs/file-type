use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110152589: FileFormat = FileFormat {
    id: 110_152_589,
    puid: "wikidata/110152589",
    name: "Serif DrawPlus Drawing, version X8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
