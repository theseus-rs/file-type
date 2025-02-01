use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111666304: FileFormat = FileFormat {
    id: 111_666_304,
    puid: "wikidata/111666304",
    name: "Liveart Sketches",
    extensions: &["lrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
