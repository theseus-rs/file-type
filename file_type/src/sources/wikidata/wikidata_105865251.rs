use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865251: FileFormat = FileFormat {
    id: 105_865_251,
    puid: "wikidata/105865251",
    name: "Palm Pilot-DB database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x44, 0x42, 0x30, 0x30, 0x44, 0x42, 0x4F, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
