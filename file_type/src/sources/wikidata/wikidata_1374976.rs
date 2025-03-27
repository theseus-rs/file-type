use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1374976: FileType = FileType {
    file_format: &FileFormat {
        id: 1_374_976,
        source_type: SourceType::Wikidata,
        name: "Progressive Graphics File",
        extensions: &["pgf"],
        media_types: &["image/x-pgf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x47, 0x46, 0x01, 0x10, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
