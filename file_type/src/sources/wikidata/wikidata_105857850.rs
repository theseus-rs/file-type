use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857850: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_850,
        source_type: SourceType::Wikidata,
        name: "ANDOS disk image",
        extensions: &["bkd", "dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA0, 0x00, 0x1E, 0x01, 0x41, 0x4E, 0x44, 0x4F, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
