use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_873,
        source_type: SourceType::Wikidata,
        name: "GNU TeXmacs Scheme",
        extensions: &["stm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x28, 0x54,
                        0x65, 0x58, 0x6D, 0x61, 0x63, 0x73, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
