use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854221: FileFormat = FileFormat {
    id: 105_854_221,
    source_type: SourceType::Wikidata,
    name: "PS/2 MicroChannel Adapter Description File (with CRLF)",
    extensions: &["adf"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
