use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967087: FileFormat = FileFormat {
    id: 27_967_087,
    puid: "wikidata/27967087",
    name: "Electronic Arts AS4/ASF Music",
    extensions: &["as4", "asf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
