use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2011664: FileFormat = FileFormat {
    id: 2_011_664,
    puid: "wikidata/2011664",
    name: "Object File Format",
    extensions: &["off"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
