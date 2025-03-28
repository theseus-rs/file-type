use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857276: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_276,
        source_type: SourceType::Wikidata,
        name: "Borland Turbo Vision Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x42, 0x48, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
