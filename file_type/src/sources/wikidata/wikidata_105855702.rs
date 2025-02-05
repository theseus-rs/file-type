use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855702: FileFormat = FileFormat {
    id: 105_855_702,
    source_type: SourceType::Wikidata,
    name: "Oberon/F Document",
    extensions: &["odc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x44, 0x4F, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x82, 0xF1, 0x44, 0x6F, 0x63,
                    0x75, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x2E, 0x53, 0x74, 0x64, 0x44, 0x6F, 0x63,
                    0x75, 0x6D, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
