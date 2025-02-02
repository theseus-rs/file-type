use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128913262: FileFormat = FileFormat {
    id: 128_913_262,
    source_type: SourceType::Wikidata,
    name: "DylanLID file format",
    extensions: &["lid"],
    media_types: &["text/x-dylan-lid"],
    internal_signatures: &[],
    related_formats: &[],
};
