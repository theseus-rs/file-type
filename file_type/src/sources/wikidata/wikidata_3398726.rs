use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3398726: FileType = FileType {
    file_format: &FileFormat {
        id: 3_398_726,
        source_type: SourceType::Wikidata,
        name: "Portable Compiled Format",
        extensions: &["pcf"],
        media_types: &["application/x-font-pcf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x66, 0x63, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
