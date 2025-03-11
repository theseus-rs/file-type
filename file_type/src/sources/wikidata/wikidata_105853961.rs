use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853961: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_961,
        source_type: SourceType::Wikidata,
        name: "Acorn ArcFS Archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
