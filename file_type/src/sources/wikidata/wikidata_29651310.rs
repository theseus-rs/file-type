use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29651310: FileFormat = FileFormat {
    id: 29_651_310,
    source_type: SourceType::Wikidata,
    name: "Pixie",
    extensions: &["pxi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
