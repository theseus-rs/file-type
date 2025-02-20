use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966964: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_964,
        source_type: SourceType::Wikidata,
        name: "4X IMA ADPCM",
        extensions: &["4xm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
