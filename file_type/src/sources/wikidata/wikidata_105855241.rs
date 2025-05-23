use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855241: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_241,
        source_type: SourceType::Wikidata,
        name: "Felix format spectra",
        extensions: &["fid"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x65, 0x6C, 0x69, 0x78, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x57, 0x69,
                        0x6E, 0x64, 0x6F, 0x77, 0x73, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
