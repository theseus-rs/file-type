use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_739,
        source_type: SourceType::Wikidata,
        name: "Kodak Digital Camera RAW image (EasyShare series)",
        extensions: &["kdc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00, 0x14,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
