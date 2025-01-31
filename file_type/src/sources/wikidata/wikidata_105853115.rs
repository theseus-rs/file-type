use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853115: FileFormat = FileFormat {
    id: 105_853_115,
    puid: "wikidata/105853115",
    name: "Elecbyte M.U.G.E.N. sound",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6C, 0x65, 0x63, 0x62, 0x79, 0x74, 0x65, 0x53, 0x6E, 0x64, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
