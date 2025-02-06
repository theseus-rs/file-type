use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130618874: FileFormat = FileFormat {
    id: 130_618_874,
    source_type: SourceType::Wikidata,
    name: "Redcode file format",
    extensions: &["cw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
