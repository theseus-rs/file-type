use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34743987: FileFormat = FileFormat {
    id: 34_743_987,
    source_type: SourceType::Wikidata,
    name: "Spark",
    extensions: &["spk"],
    media_types: &["archive"],
    internal_signatures: &[],
    related_formats: &[],
};
