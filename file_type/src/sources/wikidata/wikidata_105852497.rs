use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852497: FileFormat = FileFormat {
    id: 105_852_497,
    puid: "wikidata/105852497",
    name: "Enable SpreadSheet",
    extensions: &["ssf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
