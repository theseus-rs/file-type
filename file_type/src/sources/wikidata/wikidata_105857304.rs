use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857304: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_304,
        source_type: SourceType::Wikidata,
        name: "HydroCAD Data for prefabricated storage Chambers",
        extensions: &["hcc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x48, 0x79, 0x64, 0x72, 0x6F, 0x43, 0x41, 0x44, 0x20,
                        0x53, 0x74, 0x6F, 0x72, 0x61, 0x67, 0x65, 0x20, 0x43, 0x68, 0x61, 0x6D,
                        0x62, 0x65, 0x72, 0x20, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69,
                        0x6F, 0x6E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
