use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_154,
        source_type: SourceType::Wikidata,
        name: "BINVOX voxel file format",
        extensions: &["binvox"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x62, 0x69, 0x6E, 0x76, 0x6F, 0x78, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
