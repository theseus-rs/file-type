use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60342897: FileFormat = FileFormat {
    id: 60_342_897,
    puid: "wikidata/60342897",
    name: "Microsoft PowerPoint Show",
    extensions: &["ppsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
