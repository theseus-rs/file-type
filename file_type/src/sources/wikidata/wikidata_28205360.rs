use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205360: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_360,
        source_type: SourceType::Wikidata,
        name: "KDC (DC-Series)",
        extensions: &["kdc"],
        media_types: &["image/x-kdc", "image/x-kodak-kdc", "image/x-raw-kodak"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
