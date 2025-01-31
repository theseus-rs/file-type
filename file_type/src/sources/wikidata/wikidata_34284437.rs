use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34284437: FileFormat = FileFormat {
    id: 34_284_437,
    puid: "wikidata/34284437",
    name: "Pascal script",
    extensions: &["inc", "p", "pas", "pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
