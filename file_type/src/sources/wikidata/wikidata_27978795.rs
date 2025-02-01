use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27978795: FileFormat = FileFormat {
    id: 27_978_795,
    puid: "wikidata/27978795",
    name: "Spectrum 512 Uncompressed",
    extensions: &["spu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
