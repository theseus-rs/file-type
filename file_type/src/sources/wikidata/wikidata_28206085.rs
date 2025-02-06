use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206085: FileFormat = FileFormat {
    id: 28_206_085,
    source_type: SourceType::Wikidata,
    name: "TT High Resolution",
    extensions: &["PI7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
