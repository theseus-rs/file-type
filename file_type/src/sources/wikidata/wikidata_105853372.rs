use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853372: FileFormat = FileFormat {
    id: 105_853_372,
    puid: "wikidata/105853372",
    name: "Operation Flashpoint mission",
    extensions: &["sqm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
