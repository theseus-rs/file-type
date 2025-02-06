use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850247: FileFormat = FileFormat {
    id: 105_850_247,
    source_type: SourceType::Wikidata,
    name: "CorelDRAW Character Set",
    extensions: &["csd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x43, 0x53, 0x44, 0x43, 0x6F, 0x72, 0x65, 0x6C, 0x20, 0x44, 0x72, 0x61,
                    0x77, 0x20, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x20, 0x73,
                    0x65, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
