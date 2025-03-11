use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856263: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_263,
        source_type: SourceType::Wikidata,
        name: "OpenSolaris core dump",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0xFE, 0xC8, 0xED])],
                },
            }],
        }],
        related_formats: &[],
    },
};
