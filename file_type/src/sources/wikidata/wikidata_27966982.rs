use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27966982: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_982,
        source_type: SourceType::Wikidata,
        name: "AC1D-DC1A Packer",
        extensions: &["ac1", "ac1d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0x1D, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
