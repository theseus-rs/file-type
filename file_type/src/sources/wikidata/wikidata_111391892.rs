use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111391892: FileFormat = FileFormat {
    id: 111_391_892,
    puid: "wikidata/111391892",
    name: "Bryce2 Object",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
