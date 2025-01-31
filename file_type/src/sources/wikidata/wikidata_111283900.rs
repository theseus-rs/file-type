use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111283900: FileFormat = FileFormat {
    id: 111_283_900,
    puid: "wikidata/111283900",
    name: "G.721 4-bit (32 kbps) ADPCM format data",
    extensions: &["g721"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
