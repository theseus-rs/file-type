use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28347778: FileFormat = FileFormat {
    id: 28_347_778,
    source_type: SourceType::Wikidata,
    name: "Zeno",
    extensions: &["zeno"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
