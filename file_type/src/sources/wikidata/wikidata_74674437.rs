use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74674437: FileFormat = FileFormat {
    id: 74_674_437,
    source_type: SourceType::Wikidata,
    name: "Kindle app book info",
    extensions: &["ticr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
