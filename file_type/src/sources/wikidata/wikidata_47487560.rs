use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47487560: FileType = FileType {
    file_format: &FileFormat {
        id: 47_487_560,
        source_type: SourceType::Wikidata,
        name: "ZPAQ Archive",
        extensions: &["zpaq"],
        media_types: &["application/x-zpaq"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x6B, 0x53, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
