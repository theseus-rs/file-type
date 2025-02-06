use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866566: FileFormat = FileFormat {
    id: 105_866_566,
    source_type: SourceType::Wikidata,
    name: "ProbModelXML model",
    extensions: &["pgmx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
