use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125253619: FileFormat = FileFormat {
    id: 125_253_619,
    puid: "wikidata/125253619",
    name: "Simple interaction file",
    extensions: &["sif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
