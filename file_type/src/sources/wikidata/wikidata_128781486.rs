use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128781486: FileFormat = FileFormat {
    id: 128_781_486,
    source_type: SourceType::Wikidata,
    name: "Cryptol file format",
    extensions: &["cry"],
    media_types: &["text/x-cryptol"],
    internal_signatures: &[],
    related_formats: &[],
};
