use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850558: FileFormat = FileFormat {
    id: 105_850_558,
    puid: "wikidata/105850558",
    name: "AutoCAD Custom User Interface",
    extensions: &["cui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
