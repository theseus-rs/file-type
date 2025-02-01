use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865660: FileFormat = FileFormat {
    id: 105_865_660,
    puid: "wikidata/105865660",
    name: "WinAPE POK format",
    extensions: &["pok"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x50, 0x4F, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
