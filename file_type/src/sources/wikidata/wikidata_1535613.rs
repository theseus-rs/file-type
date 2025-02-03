use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1535613: FileFormat = FileFormat {
    id: 1_535_613,
    source_type: SourceType::Wikidata,
    name: "Styled Layer Descriptor",
    extensions: &["sld"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
