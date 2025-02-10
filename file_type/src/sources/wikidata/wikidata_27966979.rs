use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27966979: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_979,
        source_type: SourceType::Wikidata,
        name: "Organya",
        extensions: &["org"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x72, 0x67, 0x2D, 0x30, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
