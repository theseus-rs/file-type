use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_657,
        source_type: SourceType::Wikidata,
        name: "Palm PeanutReader e-book",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4E, 0x52, 0x64, 0x50, 0x50, 0x72, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
