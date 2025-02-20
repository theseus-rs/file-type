use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979399: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_399,
        source_type: SourceType::Wikidata,
        name: "Imagic Film/Picture, high resolution",
        extensions: &["ic3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x43, 0x00, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
