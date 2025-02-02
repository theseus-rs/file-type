use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861241: FileFormat = FileFormat {
    id: 105_861_241,
    source_type: SourceType::Wikidata,
    name: "GtkSourceView language definition",
    extensions: &["lang"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
