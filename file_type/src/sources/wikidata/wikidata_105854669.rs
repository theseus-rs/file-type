use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854669: FileFormat = FileFormat {
    id: 105_854_669,
    puid: "wikidata/105854669",
    name: "audfprint hash",
    extensions: &["afpt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x75, 0x64, 0x66, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x68, 0x61, 0x73, 0x68,
                    0x56, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
