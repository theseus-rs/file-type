use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49798508: FileFormat = FileFormat {
    id: 49_798_508,
    source_type: SourceType::Wikidata,
    name: "Adobe Portable Document Catalog Index File, version 2",
    extensions: &["pdx"],
    media_types: &["application/octet-stream", "text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x50, 0x44, 0x58, 0x20, 0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
