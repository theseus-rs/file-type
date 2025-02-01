use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1535613: FileFormat = FileFormat {
    id: 1_535_613,
    puid: "wikidata/1535613",
    name: "Styled Layer Descriptor",
    extensions: &["sld"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
