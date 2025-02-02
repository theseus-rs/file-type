use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61774372: FileFormat = FileFormat {
    id: 61_774_372,
    source_type: SourceType::Wikidata,
    name: "WavPack Binary",
    extensions: &["wv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
