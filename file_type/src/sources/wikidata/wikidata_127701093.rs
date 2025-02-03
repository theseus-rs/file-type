use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127701093: FileFormat = FileFormat {
    id: 127_701_093,
    source_type: SourceType::Wikidata,
    name: "Hack source code file",
    extensions: &["hack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
