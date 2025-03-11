use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5156830: FileType = FileType {
    file_format: &FileFormat {
        id: 5_156_830,
        source_type: SourceType::Wikidata,
        name: "Compound File Binary File Format",
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
