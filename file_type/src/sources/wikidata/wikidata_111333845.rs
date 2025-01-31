use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333845: FileFormat = FileFormat {
    id: 111_333_845,
    puid: "wikidata/111333845",
    name: "Rockwell ADPCM format (HotFax/Quicklink)",
    extensions: &["rif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
