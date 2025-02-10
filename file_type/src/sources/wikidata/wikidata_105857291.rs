use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857291: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_291,
        source_type: SourceType::Wikidata,
        name: "HP Phone/Database/Note database",
        extensions: &["adb", "gdb", "ndb", "pdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x63, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
