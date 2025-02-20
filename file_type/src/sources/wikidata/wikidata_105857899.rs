use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857899: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_899,
        source_type: SourceType::Wikidata,
        name: "Anex86 PC98 floppy image (720KB)",
        extensions: &["fdi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
                        0x00, 0x40, 0x0B, 0x00, 0x00, 0x02, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
                        0x02, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
