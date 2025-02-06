use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858160: FileFormat = FileFormat {
    id: 105_858_160,
    source_type: SourceType::Wikidata,
    name: "Anex86 PC98 floppy image (1.44MB)",
    extensions: &["fdi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
                    0x80, 0x16, 0x00, 0x00, 0x02, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x02, 0x00,
                    0x00, 0x00, 0x50, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
