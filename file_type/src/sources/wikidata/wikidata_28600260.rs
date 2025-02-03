use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600260: FileFormat = FileFormat {
    id: 28_600_260,
    source_type: SourceType::Wikidata,
    name: "AWD",
    extensions: &["awd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
