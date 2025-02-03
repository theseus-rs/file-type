use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100137240: FileFormat = FileFormat {
    id: 100_137_240,
    source_type: SourceType::Wikidata,
    name: "VariCAD Drawing",
    extensions: &["dwb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x87, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
