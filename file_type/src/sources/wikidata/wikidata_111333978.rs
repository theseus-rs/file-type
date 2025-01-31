use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333978: FileFormat = FileFormat {
    id: 111_333_978,
    puid: "wikidata/111333978",
    name: "Rockwell 3-bit ADPCM data",
    extensions: &["rockwell-3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
