use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959821: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_821,
        source_type: SourceType::Wikidata,
        name: "Ableton Warp Analysis",
        extensions: &["asd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x06, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
