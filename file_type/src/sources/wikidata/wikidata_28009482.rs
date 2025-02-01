use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009482: FileFormat = FileFormat {
    id: 28_009_482,
    puid: "wikidata/28009482",
    name: "SimCity 2000 Saved City",
    extensions: &["sc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
