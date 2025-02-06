use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854136: FileFormat = FileFormat {
    id: 105_854_136,
    source_type: SourceType::Wikidata,
    name: "PS/2 MicroChannel Adapter Description File (with rem)",
    extensions: &["adf"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
