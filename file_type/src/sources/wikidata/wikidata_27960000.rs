use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960000: FileFormat = FileFormat {
    id: 27_960_000,
    source_type: SourceType::Wikidata,
    name: "Perfect Clarity Audio",
    extensions: &["pca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
