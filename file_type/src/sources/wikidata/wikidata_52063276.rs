use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063276: FileFormat = FileFormat {
    id: 52_063_276,
    source_type: SourceType::Wikidata,
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
