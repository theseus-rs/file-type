use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262619: FileFormat = FileFormat {
    id: 111_262_619,
    puid: "wikidata/111262619",
    name: "Raw Yamaha DX7 32-voice data",
    extensions: &["32", "dx7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
