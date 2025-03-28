use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849935: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_935,
        source_type: SourceType::Wikidata,
        name: "EEGLAB Channel Data",
        extensions: &["ced"],
        media_types: &["text/tab-separated-values"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x75, 0x6D, 0x62, 0x65, 0x72, 0x09, 0x6C, 0x61, 0x62, 0x65, 0x6C,
                        0x73, 0x09, 0x74, 0x68, 0x65, 0x74, 0x61, 0x09, 0x72, 0x61, 0x64, 0x69,
                        0x75, 0x73, 0x09, 0x58, 0x09, 0x59, 0x09, 0x5A, 0x09, 0x73, 0x70, 0x68,
                        0x5F, 0x74, 0x68, 0x65, 0x74, 0x61, 0x09, 0x73, 0x70, 0x68, 0x5F, 0x70,
                        0x68, 0x69, 0x09, 0x73, 0x70, 0x68, 0x5F, 0x72, 0x61, 0x64, 0x69, 0x75,
                        0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
