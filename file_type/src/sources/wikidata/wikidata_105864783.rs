use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864783: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_783,
        source_type: SourceType::Wikidata,
        name: "AutoPrompt script",
        extensions: &["pmt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x64, 0x73, 0x50, 0x72, 0x6F, 0x6D, 0x70, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
