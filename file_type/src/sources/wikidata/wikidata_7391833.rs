use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7391833: FileFormat = FileFormat {
    id: 7_391_833,
    source_type: SourceType::Wikidata,
    name: "SND",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
