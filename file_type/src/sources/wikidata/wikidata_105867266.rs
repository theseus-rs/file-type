use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867266: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_266,
        source_type: SourceType::Wikidata,
        name: "Noteworthy Composer data file",
        extensions: &["nwc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x4E, 0x57, 0x5A, 0x5D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
