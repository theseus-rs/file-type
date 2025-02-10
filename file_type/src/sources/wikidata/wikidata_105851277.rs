use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851277: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_277,
        source_type: SourceType::Wikidata,
        name: "XM7 Tape image",
        extensions: &["t77"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x4D, 0x37, 0x20, 0x54, 0x41, 0x50, 0x45, 0x20, 0x49, 0x4D, 0x41,
                        0x47, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
