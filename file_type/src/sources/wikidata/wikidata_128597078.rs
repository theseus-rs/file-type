use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128597078: FileFormat = FileFormat {
    id: 128_597_078,
    source_type: SourceType::Wikidata,
    name: "AmbientTalk file",
    extensions: &["at"],
    media_types: &["text/x-ambienttalk"],
    internal_signatures: &[],
    related_formats: &[],
};
