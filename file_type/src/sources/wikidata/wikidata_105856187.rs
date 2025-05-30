use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_187,
        source_type: SourceType::Wikidata,
        name: "ClrMamePro DAT / MAME Listinfo format",
        extensions: &["dat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6C, 0x72, 0x6D, 0x61, 0x6D, 0x65, 0x70, 0x72, 0x6F, 0x20, 0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
