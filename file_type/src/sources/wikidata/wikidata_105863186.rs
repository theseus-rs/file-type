use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_186,
        source_type: SourceType::Wikidata,
        name: "Midget 3 Instruments",
        extensions: &["mis"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x69, 0x73, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
