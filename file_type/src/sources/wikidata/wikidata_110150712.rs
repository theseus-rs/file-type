use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110150712: FileFormat = FileFormat {
    id: 110_150_712,
    puid: "wikidata/110150712",
    name: "Serif DrawPlus Drawing, version 8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
