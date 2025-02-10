use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860174: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_174,
        source_type: SourceType::Wikidata,
        name: "Real C64 SID tune",
        extensions: &["sid"],
        media_types: &["audio/x-sid"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x49, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
