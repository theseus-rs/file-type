use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859603: FileFormat = FileFormat {
    id: 105_859_603,
    source_type: SourceType::Wikidata,
    name: "VisualBasic Project (EXE)",
    extensions: &["vbp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x79, 0x70, 0x65, 0x3D, 0x45, 0x78, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
