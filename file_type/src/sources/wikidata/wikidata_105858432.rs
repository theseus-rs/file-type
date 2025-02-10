use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858432: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_432,
        source_type: SourceType::Wikidata,
        name: "CAXA drawing (old)",
        extensions: &["exb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x10, 0x43, 0x61, 0x78, 0x61, 0x45, 0x62, 0x46, 0x6F, 0x72, 0x57, 0x69,
                        0x6E, 0x64, 0x6F, 0x77, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
