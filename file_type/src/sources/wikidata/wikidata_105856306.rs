use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856306: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_306,
        source_type: SourceType::Wikidata,
        name: "Skype chatsynch (old)",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x43, 0x64, 0x42, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
