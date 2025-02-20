use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_137,
        source_type: SourceType::Wikidata,
        name: "Husqvarna Viking/Pfaff Home Embroidery Format",
        extensions: &["vip"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5D, 0xFC, 0x90, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
