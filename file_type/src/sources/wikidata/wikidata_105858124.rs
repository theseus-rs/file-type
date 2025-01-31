use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858124: FileFormat = FileFormat {
    id: 105_858_124,
    puid: "wikidata/105858124",
    name: "APE ProSystem Atari 8-bit disk image (v2)",
    extensions: &["pro"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
