use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117870881: FileFormat = FileFormat {
    id: 117_870_881,
    source_type: SourceType::Wikidata,
    name: "TriGem SoftFax file",
    extensions: &["tri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
