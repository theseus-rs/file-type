use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_51917410: FileType = FileType {
    file_format: &FileFormat {
        id: 51_917_410,
        source_type: SourceType::Wikidata,
        name: "Corel Wavelet Compressed Bitmap",
        extensions: &["wi", "wvl"],
        media_types: &["application/octet-stream"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x43, 0x4F, 0x52, 0x57, 0x41, 0x56])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x57, 0x49, 0x04])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
