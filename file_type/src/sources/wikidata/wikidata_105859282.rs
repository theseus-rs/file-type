use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859282: FileFormat = FileFormat {
    id: 105_859_282,
    puid: "wikidata/105859282",
    name: "BinSCII encoded file",
    extensions: &["bsc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x4C, 0x65, 0x53, 0x74, 0x41, 0x72, 0x54, 0x66, 0x49, 0x6C, 0x45,
                    0x73, 0x54, 0x61, 0x52, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
