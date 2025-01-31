use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_82065563: FileFormat = FileFormat {
    id: 82_065_563,
    puid: "wikidata/82065563",
    name: "Euphoria Database System",
    extensions: &["edb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
