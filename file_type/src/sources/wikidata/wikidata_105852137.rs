use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852137: FileFormat = FileFormat {
    id: 105_852_137,
    puid: "wikidata/105852137",
    name: "ABC SnapGraphics Palette",
    extensions: &["sgp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x1D, 0x00, 0x47, 0x46, 0x2A, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
