use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854136: FileFormat = FileFormat {
    id: 105_854_136,
    source_type: SourceType::Wikidata,
    name: "PS/2 MicroChannel Adapter Description File (with rem)",
    extensions: &["adf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
