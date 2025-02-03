use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116941363: FileFormat = FileFormat {
    id: 116_941_363,
    source_type: SourceType::Wikidata,
    name: "Print Perfect Document",
    extensions: &["pub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
