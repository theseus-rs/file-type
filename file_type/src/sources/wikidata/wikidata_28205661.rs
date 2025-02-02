use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205661: FileFormat = FileFormat {
    id: 28_205_661,
    source_type: SourceType::Wikidata,
    name: "Acorn Sprite",
    extensions: &["acorn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
