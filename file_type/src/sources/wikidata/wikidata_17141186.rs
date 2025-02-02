use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17141186: FileFormat = FileFormat {
    id: 17_141_186,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help 2",
    extensions: &["hxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
