use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6108932: FileFormat = FileFormat {
    id: 6_108_932,
    source_type: SourceType::Wikidata,
    name: "JSGF",
    extensions: &["jsgf"],
    media_types: &["application/jsgf", "application/x-jsgf", "text/jsgf"],
    signatures: &[],
    related_formats: &[],
};
