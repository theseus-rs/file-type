use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859031: FileFormat = FileFormat {
    id: 105_859_031,
    puid: "wikidata/105859031",
    name: "GemCom Graphic bitmap",
    extensions: &["ggp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x47, 0x50, 0x46, 0x41, 0x49, 0x4B, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
