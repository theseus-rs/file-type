use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850533: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_533,
        source_type: SourceType::Wikidata,
        name: "QCad II Font",
        extensions: &["cxf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3A, 0x20, 0x20, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x51, 0x43, 0x61,
                        0x64, 0x20, 0x49, 0x49, 0x20, 0x46, 0x6F, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
