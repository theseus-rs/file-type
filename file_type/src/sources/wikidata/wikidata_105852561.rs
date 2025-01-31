use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852561: FileFormat = FileFormat {
    id: 105_852_561,
    puid: "wikidata/105852561",
    name: "Siemens mobile theme",
    extensions: &["sdt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
