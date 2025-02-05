use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855924: FileFormat = FileFormat {
    id: 105_855_924,
    source_type: SourceType::Wikidata,
    name: "Knowledge Dynamics Installer script",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                    0x74, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
