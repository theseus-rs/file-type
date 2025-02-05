use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205661: FileFormat = FileFormat {
    id: 28_205_661,
    source_type: SourceType::Wikidata,
    name: "Acorn Sprite",
    extensions: &["acorn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
