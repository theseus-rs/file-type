use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960000: FileFormat = FileFormat {
    id: 27_960_000,
    source_type: SourceType::Wikidata,
    name: "Perfect Clarity Audio",
    extensions: &["pca"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
