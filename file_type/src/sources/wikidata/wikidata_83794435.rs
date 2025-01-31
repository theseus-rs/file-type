use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83794435: FileFormat = FileFormat {
    id: 83_794_435,
    puid: "wikidata/83794435",
    name: "EclipseCrossword Word List File",
    extensions: &["ewl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
