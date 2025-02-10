use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979398: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_398,
        source_type: SourceType::Wikidata,
        name: "Imagic Film/Picture, medium resolution",
        extensions: &["ic2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x43, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
