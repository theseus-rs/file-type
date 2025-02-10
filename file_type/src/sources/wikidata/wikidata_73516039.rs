use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_73516039: FileType = FileType {
    file_format: &FileFormat {
        id: 73_516_039,
        source_type: SourceType::Wikidata,
        name: "Microsoft Private Key format",
        extensions: &["pkv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1E, 0xF1, 0xB5, 0xB0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
