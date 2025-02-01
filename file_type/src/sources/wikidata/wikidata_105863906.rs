use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863906: FileFormat = FileFormat {
    id: 105_863_906,
    puid: "wikidata/105863906",
    name: "Gen Surf map",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
