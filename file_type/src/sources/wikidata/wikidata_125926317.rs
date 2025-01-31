use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125926317: FileFormat = FileFormat {
    id: 125_926_317,
    puid: "wikidata/125926317",
    name: "SolidWorks Library Feature Part",
    extensions: &["sldlfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
