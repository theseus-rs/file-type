use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128775907: FileFormat = FileFormat {
    id: 128_775_907,
    source_type: SourceType::Wikidata,
    name: "Coq file format",
    extensions: &["v"],
    media_types: &["text/x-coq"],
    internal_signatures: &[],
    related_formats: &[],
};
