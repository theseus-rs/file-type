use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857022: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_022,
        source_type: SourceType::Wikidata,
        name: "MicroImages GeoFormula",
        extensions: &["gsf"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A, 0x0D, 0x0A, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
