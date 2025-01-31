use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864879: FileFormat = FileFormat {
    id: 105_864_879,
    puid: "wikidata/105864879",
    name: "HSQLDB configuration",
    extensions: &["properties"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x48, 0x53, 0x51, 0x4C, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
