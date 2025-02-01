use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28846076: FileFormat = FileFormat {
    id: 28_846_076,
    puid: "wikidata/28846076",
    name: "Classification Results File Format",
    extensions: &["clr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
