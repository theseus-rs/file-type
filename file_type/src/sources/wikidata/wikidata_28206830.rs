use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206830: FileFormat = FileFormat {
    id: 28_206_830,
    source_type: SourceType::Wikidata,
    name: "Palette Master",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
