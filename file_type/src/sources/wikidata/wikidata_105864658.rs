use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864658: FileFormat = FileFormat {
    id: 105_864_658,
    puid: "wikidata/105864658",
    name: "PowerBackup Job",
    extensions: &["pbj"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x6D, 0x61, 0x67, 0x69,
                    0x63, 0x3D, 0x22, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
