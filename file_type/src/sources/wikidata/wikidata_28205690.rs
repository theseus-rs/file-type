use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205690: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_690,
        source_type: SourceType::Wikidata,
        name: "AMOS Sprite Bank",
        extensions: &["abk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6D, 0x53, 0x70, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
