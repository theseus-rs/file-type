use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858872: FileFormat = FileFormat {
    id: 105_858_872,
    puid: "wikidata/105858872",
    name: "HBasic source code",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x42, 0x41, 0x53, 0x64, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
