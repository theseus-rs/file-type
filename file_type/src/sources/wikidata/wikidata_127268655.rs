use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127268655: FileFormat = FileFormat {
    id: 127_268_655,
    puid: "wikidata/127268655",
    name: "CATIA V5 Elysium Neutral File",
    extensions: &["enf_abq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
