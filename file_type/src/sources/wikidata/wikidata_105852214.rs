use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852214: FileFormat = FileFormat {
    id: 105_852_214,
    puid: "wikidata/105852214",
    name: "Kega Fusion Save State",
    extensions: &["ssx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x53, 0x4D, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
