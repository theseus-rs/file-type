use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852405: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_405,
        source_type: SourceType::Wikidata,
        name: "Tim Newport-Peace's Special Use Airspace Format",
        extensions: &["sua"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
