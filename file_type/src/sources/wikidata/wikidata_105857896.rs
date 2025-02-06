use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857896: FileFormat = FileFormat {
    id: 105_857_896,
    source_type: SourceType::Wikidata,
    name: "VAPI/ATX Atari 8-bit disk image",
    extensions: &["atx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x38, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
