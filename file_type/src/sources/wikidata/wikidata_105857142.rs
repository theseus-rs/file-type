use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_142,
        source_type: SourceType::Wikidata,
        name: "VGAPaint 386 Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x48, 0x4C, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
