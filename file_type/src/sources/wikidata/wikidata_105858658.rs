use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858658: FileFormat = FileFormat {
    id: 105_858_658,
    puid: "wikidata/105858658",
    name: "PABX Background bitmap",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x58, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
