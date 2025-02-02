use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862369: FileFormat = FileFormat {
    id: 105_862_369,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Material (v3.0)",
    extensions: &["mat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x4D, 0x41, 0x54, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
