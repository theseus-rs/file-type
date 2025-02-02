use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127605323: FileFormat = FileFormat {
    id: 127_605_323,
    source_type: SourceType::Wikidata,
    name: "Ceylon source code file",
    extensions: &["ceylon"],
    media_types: &["text/x-ceylon"],
    internal_signatures: &[],
    related_formats: &[],
};
