use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_379770: FileFormat = FileFormat {
    id: 379_770,
    puid: "wikidata/379770",
    name: "AVCHD",
    extensions: &["avchd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
