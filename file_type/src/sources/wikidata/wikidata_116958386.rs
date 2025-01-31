use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116958386: FileFormat = FileFormat {
    id: 116_958_386,
    puid: "wikidata/116958386",
    name: "Pegasus PIC",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
