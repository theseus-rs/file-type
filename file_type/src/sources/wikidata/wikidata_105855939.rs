use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855939: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_939,
        source_type: SourceType::Wikidata,
        name: "Audio Interface Library 3 Digital audio driver",
        extensions: &["dig"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x49, 0x4C, 0x33, 0x44, 0x49, 0x47, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
