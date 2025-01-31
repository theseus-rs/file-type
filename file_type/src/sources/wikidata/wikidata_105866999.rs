use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866999: FileFormat = FileFormat {
    id: 105_866_999,
    puid: "wikidata/105866999",
    name: "Neo Content file",
    extensions: &["nwp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x45, 0x4F, 0x20, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
