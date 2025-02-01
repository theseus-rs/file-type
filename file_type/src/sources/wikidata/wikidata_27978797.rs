use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27978797: FileFormat = FileFormat {
    id: 27_978_797,
    puid: "wikidata/27978797",
    name: "Spectrum 512 Compressed",
    extensions: &["spc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
