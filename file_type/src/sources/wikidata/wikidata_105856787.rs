use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856787: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_787,
        source_type: SourceType::Wikidata,
        name: "git submodule properties definition",
        extensions: &["gitmodule"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x73, 0x75, 0x62, 0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
