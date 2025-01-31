use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864460: FileFormat = FileFormat {
    id: 105_864_460,
    puid: "wikidata/105864460",
    name: "The Print Shop Deluxe Calendar",
    extensions: &["pdc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x70, 0x64, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
