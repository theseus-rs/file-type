use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858748: FileFormat = FileFormat {
    id: 105_858_748,
    puid: "wikidata/105858748",
    name: "Adobe Bridge Thumbnail cache",
    extensions: &["bct"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6C, 0x6E, 0x62, 0x74, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x66,
                    0x66, 0x6D, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
