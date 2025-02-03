use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34284437: FileFormat = FileFormat {
    id: 34_284_437,
    source_type: SourceType::Wikidata,
    name: "Pascal script",
    extensions: &["inc", "p", "pas", "pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
