use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853708: FileFormat = FileFormat {
    id: 105_853_708,
    puid: "wikidata/105853708",
    name: "Adobe Update Manager data",
    extensions: &["aum"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
