use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975882: FileFormat = FileFormat {
    id: 28_975_882,
    puid: "wikidata/28975882",
    name: "SOLIDWORKS Assembly",
    extensions: &["sldasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
