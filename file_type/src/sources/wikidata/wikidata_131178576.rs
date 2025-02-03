use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131178576: FileFormat = FileFormat {
    id: 131_178_576,
    source_type: SourceType::Wikidata,
    name: "SWIG source code file",
    extensions: &["swg"],
    media_types: &["text/swig"],
    internal_signatures: &[],
    related_formats: &[],
};
