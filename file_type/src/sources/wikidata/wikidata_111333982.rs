use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333982: FileFormat = FileFormat {
    id: 111_333_982,
    puid: "wikidata/111333982",
    name: "Rockwell 4-bit ADPCM data",
    extensions: &["rockwell-4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
