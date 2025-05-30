use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857578: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_578,
        source_type: SourceType::Wikidata,
        name: "SAM Coupe Pro-DOS disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x03, 0x81, 0x50, 0x09, 0x02, 0x01, 0x04, 0x04, 0x2A, 0x52, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x53, 0x41, 0x4D, 0x20, 0x43, 0x6F, 0x75, 0x70,
                        0x65, 0x20, 0x50, 0x72, 0x6F, 0x2D, 0x44, 0x6F, 0x73, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
