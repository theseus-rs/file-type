use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850260: FileFormat = FileFormat {
    id: 105_850_260,
    puid: "wikidata/105850260",
    name: "CodeSuite DataBase - CodeMatch",
    extensions: &["cdb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x50, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x3E, 0x43,
                    0x6F, 0x64, 0x65, 0x4D, 0x61, 0x74, 0x63, 0x68,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
