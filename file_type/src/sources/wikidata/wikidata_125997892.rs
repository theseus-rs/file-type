use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125997892: FileFormat = FileFormat {
    id: 125_997_892,
    puid: "wikidata/125997892",
    name: "Sibelius Score 3",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
