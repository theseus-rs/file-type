use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_183169: FileFormat = FileFormat {
    id: 183_169,
    puid: "wikidata/183169",
    name: "Jakarta Server Pages",
    extensions: &["jsp"],
    media_types: &["application/jsp"],
    internal_signatures: &[],
    related_formats: &[],
};
