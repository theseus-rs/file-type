use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111283919: FileFormat = FileFormat {
    id: 111_283_919,
    puid: "wikidata/111283919",
    name: "ITU G.722 7-bit (56 kbps) ADPCM format data",
    extensions: &["g722-7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
