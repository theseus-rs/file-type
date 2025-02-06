use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850558: FileFormat = FileFormat {
    id: 105_850_558,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Custom User Interface",
    extensions: &["cui"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
