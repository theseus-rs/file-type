use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126010315: FileFormat = FileFormat {
    id: 126_010_315,
    puid: "wikidata/126010315",
    name: "Sibelius Score 2020.1",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
