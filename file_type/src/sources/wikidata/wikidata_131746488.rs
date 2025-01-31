use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131746488: FileFormat = FileFormat {
    id: 131_746_488,
    puid: "wikidata/131746488",
    name: "Natron Project File",
    extensions: &["ntp"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
