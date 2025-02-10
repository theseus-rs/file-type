use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863142: FileFormat = FileFormat {
    id: 27_863_142,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2010-2012",
    extensions: &["dwg"],
    media_types: &["application/x-autocad", "image/vnd.dwg"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x32, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
