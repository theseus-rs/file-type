use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855266: FileFormat = FileFormat {
    id: 105_855_266,
    puid: "wikidata/105855266",
    name: "Elecbyte M.U.G.E.N. font",
    extensions: &["fnt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x6C, 0x65, 0x63, 0x62, 0x79, 0x74, 0x65, 0x46, 0x6E, 0x74, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
