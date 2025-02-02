use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118218029: FileFormat = FileFormat {
    id: 118_218_029,
    source_type: SourceType::Wikidata,
    name: "FotoFinish Layout",
    extensions: &["fdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
