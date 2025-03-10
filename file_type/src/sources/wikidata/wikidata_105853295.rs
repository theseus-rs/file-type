use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853295: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_295,
        source_type: SourceType::Wikidata,
        name: "Microsoft SZDD compressed (Haruhiko Okumura's LZSS)",
        extensions: &[],
        media_types: &["application/x-ms-compress-szdd"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x5A, 0x44, 0x44, 0x88, 0xF0, 0x27, 0x33, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
