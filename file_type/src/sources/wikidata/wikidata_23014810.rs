use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_23014810: FileFormat = FileFormat {
    id: 23_014_810,
    puid: "wikidata/23014810",
    name: "chr",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
