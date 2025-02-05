use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851006: FileFormat = FileFormat {
    id: 105_851_006,
    source_type: SourceType::Wikidata,
    name: "Cebra Teletext page",
    extensions: &["ttx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x45, 0x42, 0x52, 0x41, 0x20, 0x54, 0x65, 0x6C, 0x65, 0x74, 0x65, 0x78,
                    0x74, 0x20, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
