use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853823: FileFormat = FileFormat {
    id: 105_853_823,
    puid: "wikidata/105853823",
    name: "ArmPack Archive File",
    extensions: &["ap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50, 0x21, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
