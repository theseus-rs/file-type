use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_087,
        source_type: SourceType::Wikidata,
        name: "Digifont Outline Font Description",
        extensions: &["dfi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF1, 0x29])],
                },
            }],
        }],
        related_formats: &[],
    },
};
