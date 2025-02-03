use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850558: FileFormat = FileFormat {
    id: 105_850_558,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Custom User Interface",
    extensions: &["cui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
