use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866154: FileFormat = FileFormat {
    id: 105_866_154,
    puid: "wikidata/105866154",
    name: "Palm Bible+ document",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x62, 0x69, 0x62, 0x6C, 0x50, 0x50, 0x42, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
