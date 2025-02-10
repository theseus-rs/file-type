use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855194: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_194,
        source_type: SourceType::Wikidata,
        name: "Future Composer v1.4 module",
        extensions: &["fc"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x43, 0x31, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
