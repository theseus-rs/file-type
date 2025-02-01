use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117287768: FileFormat = FileFormat {
    id: 117_287_768,
    puid: "wikidata/117287768",
    name: "SigmaPlot Template File",
    extensions: &["jnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
