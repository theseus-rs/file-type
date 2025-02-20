use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865669: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_669,
        source_type: SourceType::Wikidata,
        name: "Proximity Technology Lexicon/Thesaurus",
        extensions: &["lex", "ths"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x28, 0x43,
                        0x29, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
