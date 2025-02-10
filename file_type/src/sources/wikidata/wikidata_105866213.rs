use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866213: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_213,
        source_type: SourceType::Wikidata,
        name: "FoxPro compressed dist. archive",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x50, 0x41, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
