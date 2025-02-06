use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1535613: FileFormat = FileFormat {
    id: 1_535_613,
    source_type: SourceType::Wikidata,
    name: "Styled Layer Descriptor",
    extensions: &["sld"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
