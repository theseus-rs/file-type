use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853314: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_314,
        source_type: SourceType::Wikidata,
        name: "Brother/Babylock/Bernina Home Embroidery Format",
        extensions: &["pes"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x50, 0x45, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
