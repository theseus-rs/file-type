use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111432169: FileFormat = FileFormat {
    id: 111_432_169,
    puid: "wikidata/111432169",
    name: "Hypertext Template",
    extensions: &["htt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
