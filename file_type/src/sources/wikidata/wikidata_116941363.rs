use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116941363: FileFormat = FileFormat {
    id: 116_941_363,
    puid: "wikidata/116941363",
    name: "Print Perfect Document",
    extensions: &["pub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
