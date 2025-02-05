use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17141186: FileFormat = FileFormat {
    id: 17_141_186,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help 2",
    extensions: &["hxs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
