use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205779: FileFormat = FileFormat {
    id: 28_205_779,
    source_type: SourceType::Wikidata,
    name: "Bob ray tracer bitmap",
    extensions: &["bob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
