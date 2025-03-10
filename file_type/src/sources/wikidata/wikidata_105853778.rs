use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853778: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_778,
        source_type: SourceType::Wikidata,
        name: "AR segs game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x46, 0x41, 0x53, 0x46, 0x50, 0x41, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
