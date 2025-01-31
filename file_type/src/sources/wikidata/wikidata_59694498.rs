use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59694498: FileFormat = FileFormat {
    id: 59_694_498,
    puid: "wikidata/59694498",
    name: "i2 Analysts Notebook",
    extensions: &["anb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
