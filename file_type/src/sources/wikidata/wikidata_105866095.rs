use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866095: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_095,
        source_type: SourceType::Wikidata,
        name: "Palm PocketChess deluxe games library",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x41, 0x4D, 0x45, 0x70, 0x58, 0x63, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
