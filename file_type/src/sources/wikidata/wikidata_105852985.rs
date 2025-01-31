use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852985: FileFormat = FileFormat {
    id: 105_852_985,
    puid: "wikidata/105852985",
    name: "Quattro Pro Sound",
    extensions: &["snd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x45, 0x56, 0x45, 0x02, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
