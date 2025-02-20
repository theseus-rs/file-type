use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860972: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_972,
        source_type: SourceType::Wikidata,
        name: "P-CAD ASCII Library",
        extensions: &["lia"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x41, 0x44, 0x5F, 0x41, 0x53, 0x43, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
