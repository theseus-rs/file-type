use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852234: FileFormat = FileFormat {
    id: 105_852_234,
    source_type: SourceType::Wikidata,
    name: "Vocal-Eyes Set",
    extensions: &["set"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x33, 0x56, 0x6F, 0x63, 0x61, 0x6C, 0x2D, 0x45, 0x79, 0x65, 0x73, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
