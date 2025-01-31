use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116376076: FileFormat = FileFormat {
    id: 116_376_076,
    puid: "wikidata/116376076",
    name: "Access Database Addins",
    extensions: &["mda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
