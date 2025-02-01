use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118146053: FileFormat = FileFormat {
    id: 118_146_053,
    puid: "wikidata/118146053",
    name: "Microstrip File",
    extensions: &["tl1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
