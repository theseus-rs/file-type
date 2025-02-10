use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858811: FileFormat = FileFormat {
    id: 105_858_811,
    source_type: SourceType::Wikidata,
    name: "Magick Persistent Cache bitmap",
    extensions: &["cache", "mpc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x64, 0x3D, 0x4D, 0x61, 0x67, 0x69, 0x63, 0x6B, 0x43, 0x61, 0x63, 0x68,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
