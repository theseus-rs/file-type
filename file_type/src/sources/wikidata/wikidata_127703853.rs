use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127703853: FileFormat = FileFormat {
    id: 127_703_853,
    source_type: SourceType::Wikidata,
    name: "Idris source code file",
    extensions: &["idr"],
    media_types: &["text/x-idris"],
    internal_signatures: &[],
    related_formats: &[],
};
