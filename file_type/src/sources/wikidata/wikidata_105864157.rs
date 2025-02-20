use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864157: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_157,
        source_type: SourceType::Wikidata,
        name: "MO3 module",
        extensions: &["mo3"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
