use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_912,
        source_type: SourceType::Wikidata,
        name: "Sinclair TR-DOS disk image",
        extensions: &["scl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x49, 0x4E, 0x43, 0x4C, 0x41, 0x49, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
