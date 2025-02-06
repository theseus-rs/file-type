use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864743: FileFormat = FileFormat {
    id: 105_864_743,
    source_type: SourceType::Wikidata,
    name: "ProfiCAD symbols",
    extensions: &["ppd"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x74, 0x79, 0x70,
                    0x65, 0x3D, 0x22, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x43, 0x41, 0x44, 0x20, 0x70,
                    0x70, 0x64, 0x22, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
