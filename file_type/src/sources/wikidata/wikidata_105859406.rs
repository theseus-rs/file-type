use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859406: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_406,
        source_type: SourceType::Wikidata,
        name: "QuickTime installer cache",
        extensions: &["qdat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x53, 0x49, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
