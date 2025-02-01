use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110152549: FileFormat = FileFormat {
    id: 110_152_549,
    puid: "wikidata/110152549",
    name: "Serif DrawPlus Drawing, version X6",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
