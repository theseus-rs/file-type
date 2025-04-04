use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857716: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_716,
        source_type: SourceType::Wikidata,
        name: "QuarkImmedia Document",
        extensions: &["imd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x44, 0x4F, 0x43, 0x51, 0x4F, 0x52, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
