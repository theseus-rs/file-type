use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009348: FileFormat = FileFormat {
    id: 111_009_348,
    puid: "wikidata/111009348",
    name: "PrintMaster Brochure File format",
    extensions: &["bro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
