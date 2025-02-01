use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856218: FileFormat = FileFormat {
    id: 105_856_218,
    puid: "wikidata/105856218",
    name: "DeepBurner project",
    extensions: &["dbr"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x44, 0x65, 0x65, 0x70, 0x42, 0x75, 0x72, 0x6E, 0x65, 0x72, 0x5F, 0x72,
                    0x65, 0x63, 0x6F, 0x72, 0x64, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
