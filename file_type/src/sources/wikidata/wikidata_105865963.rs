use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865963: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_963,
        source_type: SourceType::Wikidata,
        name: "Pixel Bender Kernel (with rem)",
        extensions: &["pbk"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
