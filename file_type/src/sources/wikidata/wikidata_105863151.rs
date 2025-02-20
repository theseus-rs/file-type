use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863151: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_151,
        source_type: SourceType::Wikidata,
        name: "Digitrax module",
        extensions: &["dtm", "mbm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x42, 0x20, 0x4D, 0x4F, 0x44, 0x31, 0x40,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
