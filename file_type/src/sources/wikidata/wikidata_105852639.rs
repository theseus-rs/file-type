use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852639: FileFormat = FileFormat {
    id: 105_852_639,
    puid: "wikidata/105852639",
    name: "Spectrum Prog snapshot",
    extensions: &["spg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x75, 0x6D, 0x50, 0x72, 0x6F, 0x67,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
