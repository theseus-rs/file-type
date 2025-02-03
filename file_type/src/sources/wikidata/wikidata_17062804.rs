use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17062804: FileFormat = FileFormat {
    id: 17_062_804,
    source_type: SourceType::Wikidata,
    name: "Klip",
    extensions: &["klip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
