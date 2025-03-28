use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865582: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_582,
        source_type: SourceType::Wikidata,
        name: "Print Magic Banner",
        extensions: &["pmb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x42, 0x41, 0x4E, 0x45, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
