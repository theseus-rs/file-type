use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126033191: FileFormat = FileFormat {
    id: 126_033_191,
    puid: "wikidata/126033191",
    name: "Sibelius Score 2024",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
