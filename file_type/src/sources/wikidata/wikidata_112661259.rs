use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112661259: FileFormat = FileFormat {
    id: 112_661_259,
    source_type: SourceType::Wikidata,
    name: "LightWave LScript File",
    extensions: &["ls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
