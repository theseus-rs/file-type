use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864347: FileFormat = FileFormat {
    id: 105_864_347,
    puid: "wikidata/105864347",
    name: "ProtoGen Application configuration",
    extensions: &["pva"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x50, 0x50, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
