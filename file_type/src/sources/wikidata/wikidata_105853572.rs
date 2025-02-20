use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853572: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_572,
        source_type: SourceType::Wikidata,
        name: "Atlantis Word Processor Lexicon",
        extensions: &["zlx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x4C, 0xE5, 0xBF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
