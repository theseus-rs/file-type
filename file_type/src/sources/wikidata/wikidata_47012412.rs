use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012412: FileFormat = FileFormat {
    id: 47_012_412,
    puid: "wikidata/47012412",
    name: "FoxPro Memo file format",
    extensions: &["fpt", "frt", "pjt", "vct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
