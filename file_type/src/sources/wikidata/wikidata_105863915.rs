use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863915: FileFormat = FileFormat {
    id: 105_863_915,
    source_type: SourceType::Wikidata,
    name: "MESS Floppy Image",
    extensions: &["mfi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x53, 0x53, 0x46, 0x4C, 0x4F, 0x50, 0x50, 0x59, 0x49, 0x4D, 0x41,
                    0x47, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
