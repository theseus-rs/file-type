use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853148: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_148,
        source_type: SourceType::Wikidata,
        name: "SeqBox container (v1)",
        extensions: &["sbx"],
        media_types: &["application/x-sbx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x78, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
