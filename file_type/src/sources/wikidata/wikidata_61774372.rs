use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61774372: FileFormat = FileFormat {
    id: 61_774_372,
    source_type: SourceType::Wikidata,
    name: "WavPack Binary",
    extensions: &["wv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
