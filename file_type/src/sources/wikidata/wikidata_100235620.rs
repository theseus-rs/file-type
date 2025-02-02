use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100235620: FileFormat = FileFormat {
    id: 100_235_620,
    source_type: SourceType::Wikidata,
    name: "FARO WorkSpace File",
    extensions: &["fws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
