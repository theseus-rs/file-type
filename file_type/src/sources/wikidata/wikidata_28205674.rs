use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205674: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_674,
        source_type: SourceType::Wikidata,
        name: "Alpha Microsystems BMP",
        extensions: &["bmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFF, 0x00, 0x01, 0x64, 0x00, 0x00, 0x00, 0x03, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
