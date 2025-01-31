use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863010: FileFormat = FileFormat {
    id: 105_863_010,
    puid: "wikidata/105863010",
    name: "PI Image Patterns",
    extensions: &["motivi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4F, 0x52, 0x4D, 0x50, 0x54, 0x50, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
