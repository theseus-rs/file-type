use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119786070: FileFormat = FileFormat {
    id: 119_786_070,
    puid: "wikidata/119786070",
    name: "MasterCook Tips File",
    extensions: &["mtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
