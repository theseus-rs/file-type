use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865079: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_079,
        source_type: SourceType::Wikidata,
        name: "FreePascal compiled Unit",
        extensions: &["ppu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
