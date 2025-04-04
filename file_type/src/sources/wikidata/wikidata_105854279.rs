use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854279: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_279,
        source_type: SourceType::Wikidata,
        name: "A.M.Composer 1.2 music",
        extensions: &["amc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x43, 0x20, 0x56, 0x31, 0x2E, 0x32, 0x20, 0x52, 0x45, 0x50,
                        0x4C, 0x41, 0x59, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
