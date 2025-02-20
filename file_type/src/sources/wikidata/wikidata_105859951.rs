use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859951: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_951,
        source_type: SourceType::Wikidata,
        name: "Installer VISE Mac package (old)",
        extensions: &["vct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x49, 0x53, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
