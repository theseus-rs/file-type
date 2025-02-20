use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_391,
        source_type: SourceType::Wikidata,
        name: "SlickRun MagicWord Pack",
        extensions: &["qrs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
