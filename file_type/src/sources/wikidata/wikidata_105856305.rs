use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856305: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_305,
        source_type: SourceType::Wikidata,
        name: "MacDraw drawing",
        extensions: &["drw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x44, 0x6F, 0x63, 0x44, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
