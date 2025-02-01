use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919075: FileFormat = FileFormat {
    id: 28_919_075,
    puid: "wikidata/28919075",
    name: "After Effects project template",
    extensions: &["aet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
