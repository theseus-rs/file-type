use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850381: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_381,
        source_type: SourceType::Wikidata,
        name: "idMAS Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x64, 0x4D, 0x41, 0x53, 0x63, 0x66, 0x67, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
