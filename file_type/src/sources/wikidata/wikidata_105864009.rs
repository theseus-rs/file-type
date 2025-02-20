use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864009: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_009,
        source_type: SourceType::Wikidata,
        name: "MyLittleBase database",
        extensions: &["mlb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4C, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
