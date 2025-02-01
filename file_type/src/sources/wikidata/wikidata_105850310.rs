use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850310: FileFormat = FileFormat {
    id: 105_850_310,
    puid: "wikidata/105850310",
    name: "Grand Theft Auto 3 collision data",
    extensions: &["col"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4C, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
