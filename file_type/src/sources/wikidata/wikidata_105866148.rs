use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866148: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_148,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD Pivot (gen)",
        extensions: &["pvt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x50, 0x56, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
