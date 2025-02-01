use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206490: FileFormat = FileFormat {
    id: 28_206_490,
    puid: "wikidata/28206490",
    name: "LazPaint LZP file format",
    extensions: &["lzp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x61, 0x7A, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
