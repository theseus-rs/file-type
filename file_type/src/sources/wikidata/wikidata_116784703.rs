use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116784703: FileFormat = FileFormat {
    id: 116_784_703,
    puid: "wikidata/116784703",
    name: "Form Designer Pro Form Contents",
    extensions: &["ofx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
