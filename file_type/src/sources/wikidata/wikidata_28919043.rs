use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919043: FileFormat = FileFormat {
    id: 28_919_043,
    source_type: SourceType::Wikidata,
    name: "Sony HDV",
    extensions: &["m2t"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
