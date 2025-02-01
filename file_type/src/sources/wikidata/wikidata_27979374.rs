use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979374: FileFormat = FileFormat {
    id: 27_979_374,
    puid: "wikidata/27979374",
    name: "Spruce subtitle format",
    extensions: &["stl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x46, 0x6F, 0x6E, 0x74, 0x4E, 0x61, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
