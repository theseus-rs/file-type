use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72000076: FileFormat = FileFormat {
    id: 72_000_076,
    puid: "wikidata/72000076",
    name: "File Express Index Header",
    extensions: &["ixh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
