use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865083: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_083,
        source_type: SourceType::Wikidata,
        name: "Platinen Layout Programm Layout",
        extensions: &["pla"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4C, 0x50, 0x00, 0x00, 0x82, 0x50, 0x4C, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
