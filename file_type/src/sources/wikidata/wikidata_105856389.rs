use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856389: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_389,
        source_type: SourceType::Wikidata,
        name: "Generic OLE2 / Multistream Compound",
        extensions: &[],
        media_types: &["application/x-ole-storage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
