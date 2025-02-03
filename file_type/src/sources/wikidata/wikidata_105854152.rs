use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854152: FileFormat = FileFormat {
    id: 105_854_152,
    source_type: SourceType::Wikidata,
    name: "HS2 zipped bitmap(s) archive",
    extensions: &["hsz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04, 0x0A, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
