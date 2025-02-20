use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856955: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_955,
        source_type: SourceType::Wikidata,
        name: "ExpressGraph Graph",
        extensions: &["grf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x52, 0x41, 0x50, 0x48, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
