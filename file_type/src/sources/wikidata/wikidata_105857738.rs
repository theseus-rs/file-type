use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_738,
        source_type: SourceType::Wikidata,
        name: "86Box Floppy disk image (compressed)",
        extensions: &["86f"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0x36, 0x62, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
