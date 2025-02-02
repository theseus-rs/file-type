use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857112: FileFormat = FileFormat {
    id: 105_857_112,
    source_type: SourceType::Wikidata,
    name: "XML Grammar",
    extensions: &["grxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
