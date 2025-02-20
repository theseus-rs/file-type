use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_655,
        source_type: SourceType::Wikidata,
        name: "COM+ catalog file",
        extensions: &["clb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x2B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
