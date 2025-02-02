use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854349: FileFormat = FileFormat {
    id: 105_854_349,
    source_type: SourceType::Wikidata,
    name: "LTSpice Symbol",
    extensions: &["asy"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
