use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856645: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_645,
        source_type: SourceType::Wikidata,
        name: "Verity Collection Index Descriptor",
        extensions: &["wld"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
