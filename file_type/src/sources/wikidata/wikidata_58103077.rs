use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58103077: FileFormat = FileFormat {
    id: 58_103_077,
    puid: "wikidata/58103077",
    name: "LifeTechnologies SDS",
    extensions: &["sds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
