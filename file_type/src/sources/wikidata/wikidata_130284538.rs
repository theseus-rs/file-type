use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130284538: FileFormat = FileFormat {
    id: 130_284_538,
    puid: "wikidata/130284538",
    name: "MCfunction script file",
    extensions: &["mcfunction"],
    media_types: &["text/mcfunction"],
    internal_signatures: &[],
    related_formats: &[],
};
