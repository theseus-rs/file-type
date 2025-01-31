use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130456404: FileFormat = FileFormat {
    id: 130_456_404,
    puid: "wikidata/130456404",
    name: "Beancount fileformat",
    extensions: &["beancount"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
