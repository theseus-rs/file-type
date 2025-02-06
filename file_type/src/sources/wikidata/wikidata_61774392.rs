use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61774392: FileFormat = FileFormat {
    id: 61_774_392,
    source_type: SourceType::Wikidata,
    name: "WavPack Correction File",
    extensions: &["wvc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
