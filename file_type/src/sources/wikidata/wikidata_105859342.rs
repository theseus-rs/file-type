use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859342: FileFormat = FileFormat {
    id: 105_859_342,
    puid: "wikidata/105859342",
    name: "QBX (MS Basic 7.x) Editor keyboard definition",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x43, 0x00, 0x02, 0x38, 0x3F])],
            },
        }],
    }],
    related_formats: &[],
};
