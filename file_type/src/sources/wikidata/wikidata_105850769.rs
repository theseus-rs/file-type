use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850769: FileFormat = FileFormat {
    id: 105_850_769,
    puid: "wikidata/105850769",
    name: "Kinemage protein language",
    extensions: &["kin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x74, 0x65, 0x78, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
