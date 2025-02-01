use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009607: FileFormat = FileFormat {
    id: 111_009_607,
    puid: "wikidata/111009607",
    name: "PrintMaster Letterhead File format",
    extensions: &["let"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
