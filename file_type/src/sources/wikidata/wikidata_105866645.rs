use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866645: FileFormat = FileFormat {
    id: 105_866_645,
    puid: "wikidata/105866645",
    name: "Maven Project Object Model",
    extensions: &["pom", "xml"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
