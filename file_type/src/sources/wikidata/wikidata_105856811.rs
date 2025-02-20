use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856811: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_811,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD 3D Group (v3.0)",
        extensions: &["grp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x47, 0x52, 0x50, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
