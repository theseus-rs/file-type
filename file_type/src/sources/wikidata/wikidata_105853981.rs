use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853981: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_981,
        source_type: SourceType::Wikidata,
        name: "audfprint peak",
        extensions: &["afpk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x75, 0x64, 0x66, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x70, 0x65, 0x61,
                        0x6B, 0x56, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
