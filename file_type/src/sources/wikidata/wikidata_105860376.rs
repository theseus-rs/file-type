use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860376: FileFormat = FileFormat {
    id: 105_860_376,
    puid: "wikidata/105860376",
    name: "Rebel spreadsheet (v2)",
    extensions: &["rb2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7E, 0x7E, 0x53, 0x76, 0x32, 0x7E, 0x7E])],
            },
        }],
    }],
    related_formats: &[],
};
