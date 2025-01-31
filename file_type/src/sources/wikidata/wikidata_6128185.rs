use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6128185: FileFormat = FileFormat {
    id: 6_128_185,
    puid: "wikidata/6128185",
    name: "SigmaTel Motion Video",
    extensions: &["smv"],
    media_types: &["video/x-smv"],
    internal_signatures: &[],
    related_formats: &[],
};
