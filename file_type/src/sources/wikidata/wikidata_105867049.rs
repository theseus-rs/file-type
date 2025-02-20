use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867049: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_049,
        source_type: SourceType::Wikidata,
        name: "Nintendo DS title Screen",
        extensions: &["nscr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x43, 0x53, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
