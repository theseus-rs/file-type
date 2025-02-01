use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852142: FileFormat = FileFormat {
    id: 105_852_142,
    puid: "wikidata/105852142",
    name: "Quartus Symbols data",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x59, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
