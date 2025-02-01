use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864571: FileFormat = FileFormat {
    id: 105_864_571,
    puid: "wikidata/105864571",
    name: "ICC Kodak printer image format",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x56, 0x96, 0x05])],
            },
        }],
    }],
    related_formats: &[],
};
