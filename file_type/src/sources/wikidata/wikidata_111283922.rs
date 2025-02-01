use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111283922: FileFormat = FileFormat {
    id: 111_283_922,
    puid: "wikidata/111283922",
    name: "ITU G.722 8-bit (64 kbps) ADPCM format data",
    extensions: &["g722-8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
