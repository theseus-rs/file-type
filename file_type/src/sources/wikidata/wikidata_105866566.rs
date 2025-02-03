use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866566: FileFormat = FileFormat {
    id: 105_866_566,
    source_type: SourceType::Wikidata,
    name: "ProbModelXML model",
    extensions: &["pgmx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
