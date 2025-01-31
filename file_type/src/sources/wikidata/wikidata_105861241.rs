use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861241: FileFormat = FileFormat {
    id: 105_861_241,
    puid: "wikidata/105861241",
    name: "GtkSourceView language definition",
    extensions: &["lang"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
