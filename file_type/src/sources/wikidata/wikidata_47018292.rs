use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47018292: FileFormat = FileFormat {
    id: 47_018_292,
    puid: "wikidata/47018292",
    name: "PageMaker Document file format, version 3",
    extensions: &["pm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
