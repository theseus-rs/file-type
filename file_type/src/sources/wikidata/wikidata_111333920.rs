use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333920: FileFormat = FileFormat {
    id: 111_333_920,
    puid: "wikidata/111333920",
    name: "Rockwell 2/3/4-bit ADPCM data",
    extensions: &["rockwell"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
