use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61984326: FileFormat = FileFormat {
    id: 61_984_326,
    puid: "wikidata/61984326",
    name: "Microsoft Visual FoxPro Project",
    extensions: &["pjx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
