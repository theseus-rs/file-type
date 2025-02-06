use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857870: FileFormat = FileFormat {
    id: 105_857_870,
    source_type: SourceType::Wikidata,
    name: "APE ProSystem Atari 8-bit disk image (v3)",
    extensions: &["pro"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
