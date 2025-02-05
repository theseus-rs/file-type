use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117870881: FileFormat = FileFormat {
    id: 117_870_881,
    source_type: SourceType::Wikidata,
    name: "TriGem SoftFax file",
    extensions: &["tri"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
