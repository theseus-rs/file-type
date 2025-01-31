use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858536: FileFormat = FileFormat {
    id: 105_858_536,
    puid: "wikidata/105858536",
    name: "EPOC/Symbian exported MultiBitMap",
    extensions: &["mbm"],
    media_types: &["image/x-epoc-mbm"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x00, 0x00, 0x10, 0x8A, 0x00, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
