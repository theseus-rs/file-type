use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_135997024: FileType = FileType {
    file_format: &FileFormat {
        id: 135_997_024,
        source_type: SourceType::Wikidata,
        name: "Zstandard Compression Format",
        extensions: &["zst"],
        media_types: &["application/zstd"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFD, 0x2F, 0xB5, 0x28])],
                },
            }],
        }],
        related_formats: &[],
    },
};
