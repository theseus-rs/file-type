use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61774422: FileFormat = FileFormat {
    id: 61_774_422,
    source_type: SourceType::Wikidata,
    name: "WavPack Correction File, version 5",
    extensions: &["wvc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
