use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865978: FileFormat = FileFormat {
    id: 105_865_978,
    puid: "wikidata/105865978",
    name: "Parrot ByteCode",
    extensions: &["pbc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFE, 0x50, 0x42, 0x43, 0x0D, 0x0A, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
