use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5324731: FileType = FileType {
    file_format: &FileFormat {
        id: 5_324_731,
        source_type: SourceType::Wikidata,
        name: "eXtended Triton Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
