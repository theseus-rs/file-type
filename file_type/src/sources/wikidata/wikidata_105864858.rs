use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864858: FileFormat = FileFormat {
    id: 105_864_858,
    puid: "wikidata/105864858",
    name: "PhotoLine browse index",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x68, 0x6F, 0x74, 0x6F, 0x20, 0x4C, 0x69, 0x6E, 0x65, 0x20, 0x42, 0x72,
                    0x6F, 0x77, 0x73, 0x65, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
