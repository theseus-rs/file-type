use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129988320: FileFormat = FileFormat {
    id: 129_988_320,
    puid: "wikidata/129988320",
    name: "JMESPath query file",
    extensions: &["jp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
