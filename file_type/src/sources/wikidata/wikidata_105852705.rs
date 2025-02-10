use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852705: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_705,
        source_type: SourceType::Wikidata,
        name: "GTKWave Saved session (deprecated)",
        extensions: &["sav"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x2A, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
