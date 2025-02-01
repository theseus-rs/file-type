use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865246: FileFormat = FileFormat {
    id: 105_865_246,
    puid: "wikidata/105865246",
    name: "Palm TealMovie video+audio",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x4D, 0x76, 0x69, 0x65, 0x54, 0x6C, 0x4D, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
