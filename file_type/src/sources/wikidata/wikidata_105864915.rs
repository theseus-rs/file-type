use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864915: FileFormat = FileFormat {
    id: 105_864_915,
    source_type: SourceType::Wikidata,
    name: "JASC format Palette",
    extensions: &["pal", "psppalette"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x41, 0x53, 0x43, 0x2D, 0x50, 0x41, 0x4C, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
